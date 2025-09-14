#!/bin/bash

echo "Iniciando o download dos arquivos..."
cd "$HOME" || exit

mkdir -p .esmeralda
cd .esmeralda || exit

wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.4.0/esmeralda
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.4.0/manager_db
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.4.0/sources.zip

echo "Configurando permissões..."
chmod +x ./esmeralda
chmod +x ./manager_db

echo "Descompactando recursos..."
unzip -o sources.zip 
rm sources.zip

echo "Criando atalhos na Área de Trabalho..."

BASE_DIR="$HOME/.esmeralda"
DESKTOP_DIR="$HOME/Área de trabalho"
ICON_PATH="$BASE_DIR/assets/icon/icon.png"

if [[ ! -f "$ICON_PATH" ]]; then
    echo "Erro: Ícone não encontrado em $ICON_PATH"
    exit 1
fi

EXEC_ESMERALDA="$BASE_DIR/esmeralda"
FILE_DESKTOP_ESMERALDA="$DESKTOP_DIR/esmeralda.desktop"

if [[ ! -f "$EXEC_ESMERALDA" ]]; then
    echo "Erro: Arquivo executável 'esmeralda' não encontrado em $EXEC_ESMERALDA"
    exit 1
fi

cat <<EOF > "$FILE_DESKTOP_ESMERALDA"
[Desktop Entry]
Version=1.0
Type=Application
Name=Esmeralda
Comment=Ajuda a controlar gastos e dinheiro, sabendo para onde vai o dinheiro.
Exec=$EXEC_ESMERALDA
Icon=$ICON_PATH
Terminal=false
Categories=Office;Finance;
EOF

chmod +x "$FILE_DESKTOP_ESMERALDA"
echo "Atalho 'Esmeralda' criado."

EXEC_MANAGER_DB="$BASE_DIR/manager_db"
FILE_DESKTOP_MANAGER_DB="$DESKTOP_DIR/esmeralda_db_manager.desktop"

if [[ ! -f "$EXEC_MANAGER_DB" ]]; then
    echo "Erro: Arquivo executável 'manager_db' não encontrado em $EXEC_MANAGER_DB"
    exit 1
fi

chmod +x "$FILE_DESKTOP_MANAGER_DB"

echo ""
echo "Instalação concluída com sucesso!"
echo "atalho criado na sua Área de Trabalho."

# --- Fim do Script ---
