#[allow(dead_code)]
pub fn style_global() -> String {
    "#data-invalid{
        font-size: larger;
        color: red;
        
    }
    
    #container{
        justify-content: center; 
        align-items: center; 
        text-align: center; 
        width: 100%; 
        height: 93vh; 
        display: inline-block;
    }
    
    #version{
        height: auto; 
        text-align: end;
        align-items: baseline;
    }"
    .to_string()
}
