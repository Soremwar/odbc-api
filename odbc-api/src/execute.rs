use std::intrinsics::transmute;

use widestring::U16Str;

use crate::{
    borrow_mut_statement::BorrowMutStatement, handles::Statement, parameter::Blob, CursorImpl,
    Error, ParameterRefCollection,
};

/// Shared implementation for executing a query with parameters between [`crate::Connection`],
/// [`crate::Preallocated`] and [`crate::Prepared`].
///
/// # Parameters
///
/// * `lazy_statement`: Factory for statement handle used to execute the query. We pass the
///   pass the statement lazily in order to avoid unnecessary allocating a statement handle in case
///   the parameter set is empty.
/// * `query`: SQL query to be executed. If `None` it is a assumed a prepared query is to be
///   executed.
/// * `params`: The parameters bound to the statement before query execution.
pub fn execute_with_parameters<S>(
    lazy_statement: impl FnOnce() -> Result<S, Error>,
    query: Option<&U16Str>,
    mut params: impl ParameterRefCollection,
) -> Result<Option<CursorImpl<S>>, Error>
where
    S: BorrowMutStatement,
{
    let parameter_set_size = params.parameter_set_size();
    if parameter_set_size == 0 {
        return Ok(None);
    }

    // Only allocate the statement, if we know we are going to execute something.
    let mut statement = lazy_statement()?;
    let stmt = statement.borrow_mut();
    // Reset parameters so we do not dereference stale once by mistake if we call
    // `exec_direct`.
    stmt.reset_parameters().into_result(stmt)?;
    unsafe {
        stmt.set_paramset_size(parameter_set_size)
            .into_result(stmt)?;
        // Bind new parameters passed by caller.
        params.bind_parameters_to(stmt)?;
        execute(statement, query)
    }
}

/// # Safety
///
/// * Execute may dereference pointers to bound parameters, so these must guaranteed to be valid
///   then calling this function.
/// * Furthermore all bound delayed parameters must be of type `*mut &mut dyn Blob`.
pub unsafe fn execute<S>(
    mut statement: S,
    query: Option<&U16Str>,
) -> Result<Option<CursorImpl<S>>, Error>
where
    S: BorrowMutStatement,
{
    let stmt = statement.borrow_mut();
    let need_data = if let Some(sql) = query {
        stmt.exec_direct(sql).into_result(stmt)?
    } else {
        stmt.execute().into_result(stmt)?
    };

    if need_data {
        // Check if any delayed parameters have been bound which stream data to the database at
        // statement execution time. Loops over each bound stream.
        while let Some(blob_ptr) = stmt.param_data().into_result(stmt)? {
            // The safe interfaces currently exclusively bind pointers to `Blob` trait objects
            let blob_ptr: *mut &mut dyn Blob = transmute(blob_ptr);
            let blob_ref = &mut *blob_ptr;
            // Loop over all batches within each blob
            while let Some(batch) = blob_ref.next_batch().map_err(Error::FailedReadingInput)? {
                stmt.put_binary_batch(batch).into_result(stmt)?;
            }
        }
    }

    // Check if a result set has been created.
    if stmt.num_result_cols().into_result(stmt)? == 0 {
        Ok(None)
    } else {
        // Safe: `statement` is in cursor state.
        let cursor = CursorImpl::new(statement);
        Ok(Some(cursor))
    }
}

/// Shared implementation for executing a columns query between [`crate::Connection`] and
/// [`crate::Preallocated`].
pub fn execute_columns<S>(
    mut statement: S,
    catalog_name: &U16Str,
    schema_name: &U16Str,
    table_name: &U16Str,
    column_name: &U16Str,
) -> Result<CursorImpl<S>, Error>
where
    S: BorrowMutStatement,
{
    let stmt = statement.borrow_mut();

    stmt.columns(catalog_name, schema_name, table_name, column_name)
        .into_result(stmt)?;

    // We assume columns always creates a result set, since it works like a SELECT statement.
    debug_assert_ne!(stmt.num_result_cols().unwrap(), 0);

    // Safe: `statement` is in cursor state
    let cursor = unsafe { CursorImpl::new(statement) };
    Ok(cursor)
}

/// Shared implementation for executing a tables query between [`crate::Connection`] and
/// [`crate::Preallocated`].
pub fn execute_tables<S>(
    mut statement: S,
    catalog_name: Option<&U16Str>,
    schema_name: Option<&U16Str>,
    table_name: Option<&U16Str>,
    column_name: Option<&U16Str>,
) -> Result<CursorImpl<S>, Error>
where
    S: BorrowMutStatement,
{
    let stmt = statement.borrow_mut();

    stmt.tables(catalog_name, schema_name, table_name, column_name)
        .into_result(stmt)?;

    // We assume columns always creates a result set, since it works like a SELECT statement.
    debug_assert_ne!(stmt.num_result_cols().unwrap(), 0);

    // Safe: `statement` is in Cursor state.
    let cursor = unsafe { CursorImpl::new(statement) };

    Ok(cursor)
}
