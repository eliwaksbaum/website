@echo off

"C:\Program Files (x86)\WinSCP\WinSCP.com" ^
  /log="C:\Users\Eli\WinSCP.log" /ini=nul ^
  /command ^
    "open sftp://eli@10.8.3.1:18162/ -hostkey=""ssh-ed25519 255 G7WqzGCTs6gmaLmTFGTWJBMXdBQaR/JeVkfDwbLXQtE="" -privatekey=""C:\Users\Eli\.ssh\id_ed25519.ppk"" -rawsettings TryAgent=0 AuthKI=0" ^
    "synchronize remote -mirror C:\Users\Eli\Desktop\ssg\.com /var/www/staging" ^
    "exit"

set WINSCP_RESULT=%ERRORLEVEL%
if %WINSCP_RESULT% equ 0 (
  echo Success
) else (
  echo Error
)

exit /b %WINSCP_RESULT%
