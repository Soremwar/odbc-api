' This is a monstruosity but...Windows
Set WshShell = WScript.CreateObject("WScript.Shell")

WshShell.Run "C:\sqlite_connector.exe"
WScript.Sleep(5000)

Wscript.Echo "Running installer"
WshShell.AppActivate "SQLite3 ODBC Driver for Win64 Setup"
WshShell.SendKeys "{ENTER}"
WshShell.SendKeys "{ENTER}"
WshShell.SendKeys "{ENTER}"
WshShell.SendKeys "{ENTER}"
WScript.Sleep(10000)
WshShell.SendKeys "{ENTER}"
