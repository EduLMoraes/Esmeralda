#[allow(dead_code)]
pub fn style_login() -> String {
    "body{
        background-color: rgb(4, 15, 42);
        color: white;
        font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
    }
    #login{
        justify-content: center; 
        align-items: center; 
        height: auto; 
        text-align: center; 
        background-color: transparent; 
        border: 0; 
        width: auto; 
        display: inline-block; 
        margin-left: 38%; 
        margin-right: 38%;
        height: 65vh; 
        margin-top: 15%;
    }
    form{
        display: inline-grid;
        height: 150px;
        margin-top: 0;
        margin-left: 0;
        width: auto;
        padding: 20px;
    }
    input{
        border-style: solid;
        border-color: rgb(47, 125, 151);
        border-width: 5px;
        border-radius: 15px;
        padding-left: 30px;
        height: 30px;
        width: 230px;
    }
    input::selection, input::after, input::marker, input:focus{
        border-color: transparent;
        box-shadow: 0 0 0 0;
        border-style: solid;
        border-color: green;
        border-width: 5px;
        border-radius: 15px;
        outline: 0;
    }
    #esmeralda{
        color: greenyellow;
    }
    #submit{
        color: black;
        background-color: rgb(132, 218, 132);
        width: 50px;
        height: 30px;
        width: 90px;
        border-radius: 20px;
        border-color: transparent;
        font-size: large;
        font-weight: bold;
    }
    #register{
        width: 100px;
        border-radius: 20px;
        border-color: transparent;
        color: green;
    }
"
    .to_string()
}
