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

    #div-right{
        max-height: 60%;
    }

    #div-table{
        height: 90%;
        overflow: auto;
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
    
    table{
        border-collapse: collapse;
    }
    
    
    th{
        border-top: 0px solid black;
        color: black;
        text-align: center;
        text-align: center;
        background-color: whitesmoke;
    }

    td{
        padding-left: 10px;
        padding-right: 10px;
        text-align: center;
        align-items: center;
    }
    
    #with-button{
        padding: 0;
        max-width: 100px;
    }
    
    tr{
        background-color: rgb(224, 224, 224);
        color: rgb(0, 0, 0);
        border-top: 1px solid black;
    }
    
    #search-bar{
        width: 60%;
        border-radius: 50px;
        padding: 5px;
    }

    #head-table{
        background-color: whitesmoke;
        font-size: 14px;
        text-align: left;
        color: black;
        width: 100%;
    }
    
    #stt-neg, #stt-pos{
        border-radius: 100px;
        width: 10px;
        height: 10px;
        animation: blinks infinite;
    }
    #stt-neg{
        background-color: red;
        border-color: red;
        border-style: solid;
        animation-duration: 0.5s;
    }
    #stt-pos{
        background-color: green;
        border-color: green;
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
        color: green;
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

    #col-title, #col-description{
        text-align: left;
        max-width: 150px;
    }

    #col-debtor-value{
        max-width: 70px;
    }

    #col-nature{
        text-align: left;
    }

    #col-value-head{
    }
    
    #col-title, #col-description, #col-nature, #col-name, #col-value, #col-id {
        color: black;
        background-color: transparent;
        table-layout: fixed;
        height: 40px;
        transition: height 0.4s;
        
        max-width: 150px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        transition: all ease 0.7s;
    }

    #col-title:hover, #col-description:hover, #col-nature:hover, #col-debtor-value:hover, #col-name:hover, #col-value:hover, #col-id:hover {
        height: auto;
        overflow: auto;
        text-justify: justify;
        text-overflow: ellipsis;
        white-space: initial;
    }

    #col-name{
        color: black;
        background-color: transparent;
        table-layout: fixed;
        height: 40px;
        transition: height 0.4s;
        text-align: left;

        max-width: 80px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        transition: all ease 0.7s;
    }

    #col-name:hover{
        height: auto;
        overflow: auto;
        text-justify: justify;
        text-overflow: ellipsis;
        white-space: initial;
    }

    #table-debtors{
        margin-top: 20px;
    }
    "
    .to_string()
}
