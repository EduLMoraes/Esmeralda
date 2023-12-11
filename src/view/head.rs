pub fn head_style() -> String{
    "<style>
        body{
            background-color: rgb(0, 0, 50);
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
            height: auto; 
            margin-top: 15%;
        }
        form{
            display: inline-grid;
            background-color: whitesmoke;
            height: 150px;
            margin-top: 0;
            margin-left: 0;
            width: auto;
            border: black;
            padding: 20px;
        }
    </style>".to_string()
}