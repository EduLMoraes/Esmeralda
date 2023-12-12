pub fn login() -> Result<(), ErrorLogin>{

}

enum ErrorLogin{
    ErrorAuthenticate(ErrorLog<'static>),
    ErrorValueInvalid(ErrorLog<'static>)
}