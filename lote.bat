::::::::::::::::::::::::::::::::::::::::::::
:: Elevate.cmd - Version 4
:: Automatically check & get admin rights
::::::::::::::::::::::::::::::::::::::::::::
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
ECHO args = args ^& strArg ^& " "  >> "%vbsGetPrivileges%"
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
if '%1'=='ELEV' (del "%vbsGetPrivileges%" 1>nul 2>nul  &  shift /1)

::::::::::::::::::::::::::::
::START
::::::::::::::::::::::::::::
REM Run shell as admin (my) 



ECHO =============================
ECHO acessando diretorio pgsql
ECHO =============================
     cd Documents\Esmeralda\pgsql\bin     

ECHO =============================
ECHO verifica servico
ECHO =============================
    net stop pgsql
    pgsqld -remove pgsql


ECHO =============================
ECHO Definindo o pgsql como um serviÃ§o
ECHO =============================
     pgsqld --install pgsql

ECHO =============================
ECHO iniciando servico
ECHO =============================
      sc start pgsql

ECHO =============================
ECHO ALTERA USUARIO ROOT
ECHO =============================
    pgsql -uroot   -e  UPDATE   pgsql.user   SET    Password = PASSWORD('postgres')    WHERE    User = 'postgres';    FLUSH PRIVILEGES;

ECHO =============================
ECHO  carregando o banco de dados ...
ECHO =============================
   pgsql -uroot   -e "CREATE DATABASE esmeralda;"
   pgsql -uroot   use esmeralda;

ECHO =============================
ECHO  Reiniciando servico.
ECHO =============================
    net stop pgsql
    sc start pgsql

ECHO =============================
ECHO  Carregando as Tabelas... 
ECHO =============================   
     pgsql -uroot  esmeralda < construct_db.sql


echo
echo banco carregado com sucesso !
echo
echo
echo