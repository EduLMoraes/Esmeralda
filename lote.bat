@echo off
CLS
ECHO.
ECHO =============================
ECHO Running Admin shell
ECHO =============================

:init
setlocal DisableDelayedExpansion
set cmdInvoke=1
set winSysFolder=System32
set "batchPath=%~0"
for %%k in (%0) do set batchName=%%~nk
set "vbsGetPrivileges=%temp%\OEgetPriv_%batchName%.vbs"
setlocal EnableDelayedExpansion

:checkPrivileges
NET FILE 1>NUL 2>NUL
if '%errorlevel%' == '0' ( goto gotPrivileges ) else ( goto getPrivileges )

:getPrivileges
if '%1'=='ELEV' (echo ELEV & shift /1 & goto gotPrivileges)
ECHO.
ECHO **************************************
ECHO Invoking UAC for Privilege Escalation
ECHO **************************************

ECHO Set UAC = CreateObject^("Shell.Application"^) > "%vbsGetPrivileges%"
ECHO args = "ELEV " >> "%vbsGetPrivileges%"
ECHO For Each strArg in WScript.Arguments >> "%vbsGetPrivileges%"
ECHO args = args ^& strArg ^& " " >> "%vbsGetPrivileges%"
ECHO Next >> "%vbsGetPrivileges%"

if '%cmdInvoke%'=='1' goto InvokeCmd 

ECHO UAC.ShellExecute "!batchPath!", args, "", "runas", 1 >> "%vbsGetPrivileges%"
goto ExecElevation

:InvokeCmd
ECHO args = "/c """ + "!batchPath!" + """ " + args >> "%vbsGetPrivileges%"
ECHO UAC.ShellExecute "%SystemRoot%\%winSysFolder%\cmd.exe", args, "", "runas", 1 >> "%vbsGetPrivileges%"

:ExecElevation
"%SystemRoot%\%winSysFolder%\WScript.exe" "%vbsGetPrivileges%" %*
exit /B

:gotPrivileges
setlocal & cd /d %~dp0
if '%1'=='ELEV' (del "%vbsGetPrivileges%" 1>nul 2>nul & shift /1)

::::::::::::::::::::::::::::
::START
::::::::::::::::::::::::::::
REM Run shell as admin (my) > log.txt

ECHO =============================
ECHO acessando diretorio postgresql
ECHO =============================
cd %HOMEDRIVE%%HOMEPATH%
cd .esmeralda     
cd pgsql
cd bin

ECHO =============================
ECHO Tornando pgsql um servico
ECHO =============================
set POSTGRES_BIN_PATH="%HOMEDRIVE%%HOMEPATH%\.esmeralda\pgsql\bin"
set PATH=%PATH%;%POSTGRES_BIN_PATH%
set SERVICE_NAME=PostgreSQL
set DATA_DIRECTORY="%HOMEDRIVE%%HOMEPATH%\.esmeralda\pgsql\data"

cd /D %POSTGRES_BIN_PATH%
pg_ctl register -N "%SERVICE_NAME%" -D %DATA_DIRECTORY%

cacls "%HOMEDRIVE%%HOMEPATH%\.esmeralda\pgsql\data" /E /T /C /G postgres:F

ECHO =============================
ECHO iniciando servico
ECHO =============================
sc %DATA_DIRECTORY%
initdb -D %DATA_DIRECTORY% -U "postgres"
pg_ctl start -D %DATA_DIRECTORY%

ECHO =============================
ECHO ALTERA USUARIO POSTGRES
ECHO =============================

psql -U postgres -c "ALTER USER postgres WITH PASSWORD 'postgres';"

ECHO =============================
ECHO criando o banco de dados ...
ECHO =============================
psql -U postgres -c "CREATE DATABASE esmeralda;"

ECHO =============================
ECHO Carregando as Tabelas... 
ECHO =============================   
psql -U postgres -d esmeralda -f construct_db.sql


@echo off
start /b "" cmd /c net start "%SERVICE_NAME%" >nul  2>&1

exit