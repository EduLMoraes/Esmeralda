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
REM Run shell as admin (my) 

ECHO =============================
ECHO acessando diretorio postgresql
ECHO =============================
cd\Esmeralda     
cd pgsql
cd bin

ECHO =============================
ECHO Tornando pgsql um servico
ECHO =============================
pg_ctl register -N "PostgreSQL" -D "C:\Esmeralda\pgsql\data" start

ECHO =============================
ECHO iniciando servico
ECHO =============================
sc start PostgreSQL

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

echo banco carregado com sucesso !
