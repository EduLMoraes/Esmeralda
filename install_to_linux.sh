echo "Download de arquivos..."
cd $HOME
mkdir .esmeralda
cd .esmeralda
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.4.0/esmeralda
chmod +x ./esmeralda
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.4.0/sources.zip

echo "Descompactando..."
unzip sources.zip
rm sources.zip

echo "Criando atalho..."
NOME="Esmeralda"
COMENTARIO="The purpose of the Emerald is to help control spending and money, knowing where the money goes, how much can be spent, who made the debt and the total of that debt."
EXEC="$HOME/.esmeralda/esmeralda"
ICON="$HOME/.esmeralda/assets/icon/icon.png"
FILE_DESKTOP="$HOME/Área\ de\ trabalho/esmeralda.desktop"

if [[ ! -f "$EXEC" ]]; then
    echo "Erro: Arquivo executável não encontrado em $EXEC"
    exit 1
fi

if [[ ! -f "$ICON" ]]; then
    echo "Erro: Ícone não encontrado em $ICON"
    exit 1
fi

touch $FILE_DESKTOP
cat <<EOF >> "$FILE_DESKTOP"
[Desktop Entry]
Version=1.0
Type=Application
Name=$NOME
Comment=$COMENTARIO
Exec=$EXEC
Icon=$ICON
Terminal=false
EOF

chmod +x "$FILE_DESKTOP"
echo "Instalação concluída com sucesso!."