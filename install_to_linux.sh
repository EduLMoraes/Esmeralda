cd $HOME
mkdir .esmeralda
cd .esmeralda
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.0.0/esmeralda_1.0.0_amd64.deb
sudo dpkg -i esmeralda_1.0.0_amd64.deb
wget https://github.com/EduLMoraes/Esmeralda/releases/download/v1.0.0/sources.zip
unzip sources.zip
rm sources.zip
