<h1 style="color: #37af03"> <b>$$ Esmeralda $$</b></h1> 

<div style="
  background-color: #1f1f1f;
  ">
  <img src="./assets/images/icon.png" style="
    display: flex;
    margin: auto;
    width: 175px;
  "/>
</div>

<h3 style="text-align: justify">
<b style="color: #37af03">Esmeralda</b> é um software de controle financeiro para desktop em contínua evolução para um mais completo e 
<b style="color: #eeaa00"> gratuito e seguro</b>. Buscando um aumento na qualidade da gestão financeira pessoal, com uma interface clara e de fácil
entendimento contendo as naturezas dos gastos separadas, os devedores que forem 
registrados, cadastro e login para acesso ao sistema.
</h3>

## Sumário:
  1.    [Detalhes](#details)
  2.    [Features](./features.md)
  3.    [Licença](./LICENSE) 
  4.    [Passo a passo para usar](#how-use)
  5.    [Imagens](#images)
  6.    [Créditos](#credits)

# *Detalhes*: <section id="details"/>

### *Versão* 🤖
`v1.3.1`

<h2> Tecnologias </h2>
<p>
<input type="checkbox" checked> GTK4    <br>
<input type="checkbox" checked> SQLite  <br>
<input type="checkbox" checked> Rust    <br>
<input type="checkbox" checked> CSS     <br>

</p>

## Requisitos Mínimos
<table>
  <tr>
    <td> Núcleos de CPU </td>
    <td> 2</td>
  </tr>
  <tr>
    <td> Memória RAM </td>
    <td> 2Gb</td>
  </tr>
  <tr>
    <td> Armazenamento </td>
    <td> 100mb</td>
  </tr>
  <tr>
    <td> OS </td>
    <td> Linux / Windows 10</td>
  </tr>
</table>

### *Referências*
Todos os ícones são pertencentes à [Flaticon](https://www.flaticon.com/br/).
<br>
O projeto Esmeralda dá todos os créditos pelos ícones à [Flaticon](https://www.flaticon.com/br/) e seus artistas.

### Features
[ → Veja as mudanças aqui ←](./features.md) 

### Segurança
Confira [aqui](./SECURITY.md) quais versões tem suporte e como relatar um problema encontrado.  

### *Licença* 📜
O Software está submetido ao licenciamento [GNU Affero General Public License v3.0](./LICENSE).

## Como usar? <section id="how-use"/>

O aplicativo esta disponível para os sistemas operacionais listados a baixo:
<div style = "display: flex; margin: auto; width: 200px; font-size: larger;">

|Sistema <br> operacional|Versão <br> Disponível|
|-------------------|----------|
|Windows 10         |  0.1.5+  |
|Linux              |  1.1.0 < |
|MacOs              |    ❌    |

</div>

### Instalação:

### Como instalar no Windows 10

#### Passo 1 - Baixe o instalador disponível [Aqui](https://github.com/EduLMoraes/Esmeralda/releases/download/v1.0.0/Esmeralda_installer-v1.0.0.exe) ou em [releases](https://github.com/EduLMoraes/Esmeralda/releases/)

#### Passo 2 - Rode o instalador, por falta de assinatura o windows pode pedir  confirmação para executar o programa, basta clicar em `saiba mais > executar mesmo assim`. 

``Obs.: Marcar para gerar atalho se estiver desmarcada.``

#### Passo 3 - Ao finalizar a instalação, rode o atalho que deve ter sido gerado na área de trabalho.

---

### Como instalar no Linux *Em construção*
#### Passo 1 - Baixe o executável do esmeralda aqui [Linux](https://github.com/EduLMoraes/Esmeralda/releases/download/v1.0.0/esmeralda)

#### Passo 2 - Rode estes comandos:
```bash
  curl -fsSL https://raw.githubusercontent.com/EduLMoraes/Esmeralda/main/install_to_linux.sh | bash
```

---
#### 3 - Usando
  3.1 - Agora com tudo baixado e configurado, basta rodar ele, ir na tela de cadastro e criar seu login, logar e seguir usando-o normalmente, como mostrado em [imagens](#images)

---

## *Imagens* 📷 <section id = "images" />


### *Login*
A tela de login será a primeira tela a ser vista ao abrir o programa,
ela poderá te redirecionar para a tela de cadastro caso não haja conta
ou para a principal quando realizar o login.

### *Cadastro*
A tela de cadastro te permite criar uma conta, caso tente cadastrar uma
conta já existente, ela acusará falha no cadastro, do contrário, ela 
devolverá uma mensagem de sucesso e te pedirá para ir para a tela de login.
<img src="./assets/gif/login.gif" style="display: flex; margin: auto;">
___
### *Homepage || Página principal*
Aqui é onde a mágica acontece.
 ### Adicionando conta.
  
  Ao adicionar uma conta ela aparecerá na lista à direita da tela
  e um resumo dos Devedores aparecerão na lista à esquerda.

 <img src="./assets/gif/added.gif" style="display: flex; margin: auto;"> 
 
 ___

 ### Pagando conta.

  Ao pagar uma conta, ele contabilizará uma parcela paga por vez, ao
  pagar todas as parcelas ela será dada como paga e o status mudará
  para positivo, significando que a conta está paga, isso reduzirá
  o valor total de dívida do devedor correspondente.
<img src="./assets/gif/pay.gif" style="display: flex; margin: auto;">
 
 ___


### *Como contribuir com o projeto*
Para contribuir com o projeto basta ler a documentação em [/doc](./doc/) e clonar o projeto com a seguinte linha
```git
git clone "https://github.com/EduLMoraes/Esmeralda.git"
```
E usar um repositório em seu github, ao finalizar as mudanças feitas por
ti, basta criar um pull request *(PR)* descrevendo o que foi modificado,
porquê foi modificado.

`Em caso de nova funcionalidade, documente-a e crie teste automatizado, se possível.`

Você pode rodar o projeto (necessário rust instalado) usando:
```bash
cargo run
```

Para rodar os testes:
```bash
make test
```

Para enviar suas alterações:
```bash
make git
```

### *Contribuidores*: <section id="credits"/>
<table>
  <tr>
     <td align="center"><a href="https://github.com/EduardoMoreaes"><img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/88555769?v=4" width="100px;" alt=""/><br /><sub><b>Eduardo Lopes de Moraes</b></sub></a><br /><a href="https://github.com/EduardoMoreaes" title="Desenvolvedor">👨‍🚀</a></td>
  </tr>
<table>
