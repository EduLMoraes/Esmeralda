use std::fmt;

/// ErrorLog irá padronizar a saída de erros do nosso programa,
/// para facilitar a localização do defeito e do problema nós
/// usamos o titulo para descrição breve do erro, o codigo para
/// identificar qual erro está dando e o arquivo de onde o erro
/// foi emitido.
///
/// Os códigos de erros são os seguintes:
/// - 816: Tipo de dado inválido para o banco de dados.
/// - 812: Erro de tabela inexistente no banco de dados.
/// - 808: Erro ao inserir dados no banco de dados.
/// - 804: Erro ao procurar o banco de dados e não encontrá-lo.
/// - 802: Erro ao configurar o banco de dados.
/// - 800: Erro em outro arquivo que teve inicio no banco de dados.
/// - 500: Erro em biblioteca de terceiros.
/// - 404: Erro de página não encontrada.
/// - 306: Erro na entrada de dados na função, tipo inválido.
/// - 305: Erro na entrada de dados na função, dado inválido ou incompleto.
/// - 304: Erro ao procurar um módulo e não encontrá-lo.

#[derive(Debug, PartialEq)]
pub struct ErrorLog<'a> {
    pub title: &'a str,
    pub code: i32,
    pub file: &'a str,
}

impl<'a> fmt::Display for ErrorLog<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error: {}\n -> Code: {}\n -> File: {}\n",
            self.title, self.code, self.file
        )
    }
}

use crate::Error;

#[derive(Error, Debug, PartialEq)]
#[allow(dead_code)]
pub enum DataBaseError {
    #[error("Config error")]
    GetConfigError(ErrorLog<'static>),

    #[error("Require pool error")]
    CreatePoolError(ErrorLog<'static>),

    #[error("Add user not working")]
    AddUserError(ErrorLog<'static>),

    #[error("Config error")]
    GetUserError(ErrorLog<'static>),

    #[error("DataType not Acept")]
    DataTypeInvalid(ErrorLog<'static>),
}

#[allow(dead_code)]
#[derive(Error, Debug, PartialEq)]
pub enum ControlError {
    #[error("Error of module extern")]
    ErrorExternDB(DataBaseError),

    #[error("Add user error")]
    ErrorExtern(ErrorLog<'static>),

    #[error("Add user error")]
    ErrorToAddUser(ErrorLog<'static>),

    #[error("Authenticate error")]
    ErrorAuthenticate(ErrorLog<'static>),

    #[error("Error of value invalid")]
    ErrorValueInvalid(ErrorLog<'static>)
}
