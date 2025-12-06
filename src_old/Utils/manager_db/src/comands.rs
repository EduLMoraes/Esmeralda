use super::*;

#[derive(Subcommand)]
pub enum DbCommand {
    /// Cria um novo banco de dados
    Create {
        /// Versão específica ou última versão
        #[arg(short, long)]
        last_version: String,
    },
    /// Atualiza o banco de dados para uma nova versão
    Update {
        /// Versão para qual atualizar
        #[arg(short, long)]
        version: String,
    },
    /// Faz backup do banco de dados
    Backup {
        /// Caminho para salvar o backup
        #[arg(short, long)]
        path: String,
    },
    /// Restaura o banco de dados a partir de um arquivo de backup
    Restore {
        /// Caminho do arquivo de backup
        #[arg(short, long)]
        backup_file: String,
    },
    /// Deleta o banco de dados
    Delete,
}
