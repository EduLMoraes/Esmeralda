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

#[allow(dead_code)]
pub fn style_register() -> String {
    "body{
        background-color: rgb(4, 15, 42);
        color: white;
        font-family: 'Gill Sans', 'Gill Sans MT', Calibri, 'Trebuchet MS', sans-serif;
    }
    #register{
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
        margin-top: 15%;
    }
    form{
        display: inline-grid;
        height: 150px;
        margin-top: 0;
        margin-left: 0;
        align-items: center;
        text-align: center;
        width: auto;
        padding: 20px;
    }
    input{
        border-style: solid;
        border-color: rgb(47, 125, 151);
        border-width: 5px;
        border-radius: 15px;
        padding-left: 30px;
        margin-bottom: 10px;
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
        width: 200px;
        margin-top: 20px;
        border-radius: 20px;
        border-color: transparent;
        font-size: large;
        font-weight: bold;
        margin-left: 13%;
    }
    #login{
        width: auto;
        border-radius: 20px;
        border-color: transparent;
        color: green;
    }
    
    #input-invalid{
        border-color: red;
    }"
    .to_string()
}

#[allow(dead_code)]
pub fn style_home() -> String {
    "body{
        background-color: rgb(29, 29, 29);
        font-size: 1rem;
    }
    #container{
        background-color: rgb(5, 10, 20);
        color: white;
        font-family:Georgia, 'Times New Roman', Times, serif;
        display: flex;
        flex-wrap: wrap;
        align-items:flex-start;
        text-align: left;
    
    }
    
    #button-order{
        border: 0;
        background-color: transparent;
        width: 100%;
        text-align: center;
        margin: 0;
    }
    
    h4{
        text-decoration: underline;
    }
    
    
    #table_counts{
        border-color: greenyellow;
        border-style: solid;
        border-radius: 20px;
        border-width: 2px;
        height: 600px;
        display: block;
    }
    
    td{
        border-color: rgb(0, 0, 0);
        border-width: 1px;
        border-style: solid;
        padding-left: 10px;
        padding-right: 10px;
        border-top: 0;
        border-bottom: 1px;
        text-align: center;
        align-items: center;
    }
    
    #with-button{
        padding: 0;
        max-width: 150px;
    }
    
    tr{
        background-color: rgb(224, 224, 224);
        color: rgb(0, 0, 0);
    }
    
    #head-table{
        background-color: whitesmoke;
        border-color: green;
        border-width: 1px;
        border-style: solid;
        border-top: 0;
        border-bottom: 0;
        color: black;
    }
    
    #stt-neg, #stt-pos{
        border-radius: 100px;
        width: 2vw;
        height: 2vw;
        
        /* min-height: 1.2vh;
        min-width: 1.2vh;
        max-width: 1.2vh;
        max-height: 1.2vh; */
        animation: blinks infinite;
    }
    #stt-neg{
        background-color: red;
        border-color: red;
        border-style: solid;
        animation-duration: 0.5s;
    }
    #stt-pos{
        background-color: greenyellow;
        border-color: greenyellow;
        border-style: solid;
        animation-duration: 0.6s;
    }
    
    @keyframes blinks{
        0%{
        }
        50%{
            background-color: transparent;
        }
    }
    #name{
        color: greenyellow;
        width: 100%;
        margin-left: 10px;
    }
    
    button{
        padding: 3px;
        padding-left: 10px;
        padding-right: 10px;
        background-color: white;
        border-width: 1px;
        border-color: blue;
        font-family:'Franklin Gothic Medium', 'Arial Narrow', Arial, sans-serif;
        font-size: larger;
        border-radius: 10px;
        margin-left: 20px;
    }
    
    button:active{
        background-color: blue;
    }
    
    #div-options{
        margin-top: 20px;
        display: inline;
        flex-wrap: wrap;
    }
    
    
    
    #active{
        background-color: rgb(11, 39, 76);
        color: wheat;
    }
    form{
        display: inline-grid;
        height: 150px;
        margin-top: 0;
        margin-left: 0;
        width: auto;
        padding: 20px;
    }
    #div-body{
        display: flex;
        justify-content: space-evenly;
        width: 100%;
        height: 90%;
    }
    
    #div-most{
        display: flex;
        flex-direction: column;
    }
    
    #td-most{
        text-align: left;
    }
    
    input{
        height: 20px;
        border-radius: 8px;
    }
    
    #devedor{
        width: 100px;
    }
    #value{
        width: 70px;
    }
    #installments{
        width: 40px;
    }
    #payment{
        height: 15px;
        width: 15px;
    }
    
    #move-page{
        width: 100%;
        display: flex;
        justify-content: space-evenly;
        text-align: center;
    }
    
    th{
        width: 100%;
    }
    
    #col-id{
        width: 20px;
    }
    
    #col-value{
        text-align: right;
        max-width: 70px;
    }

    #col-name{
        text-align: left;
        max-width: 120px;
    }

    #col-title, #col-description{
        text-align: left;
        max-width: 150px;
    }

    #col-debtor-value{
        max-width: 70px;
    }

    #table-debtors{
        margin-top: 20px;
    }
    "
    .to_string()
}
