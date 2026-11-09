//! Módulo de Internacionalização (i18n) e Detecção de Idioma do Sistema.
//!
//! Detecta automaticamente o idioma do sistema operacional do usuário
//! (ex.: Windows configurado em inglês -> UI em inglês, português -> UI em português).
//! Permite também alternar manualmente entre Auto, Inglês e Português nas Configurações.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LanguageMode {
    #[default]
    Auto,
    PtBr,
    EnUs,
}

impl LanguageMode {
    pub const ALL: &'static [LanguageMode] = &[LanguageMode::Auto, LanguageMode::PtBr, LanguageMode::EnUs];

    pub fn label(&self) -> &'static str {
        match self {
            LanguageMode::Auto => "Automático (detectar do sistema)",
            LanguageMode::PtBr => "Português (Brasil)",
            LanguageMode::EnUs => "English (US)",
        }
    }

    pub fn label_en(&self) -> &'static str {
        match self {
            LanguageMode::Auto => "Automatic (detect from system)",
            LanguageMode::PtBr => "Português (Brasil)",
            LanguageMode::EnUs => "English (US)",
        }
    }
}

impl std::fmt::Display for LanguageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Lang {
    Pt,
    #[default]
    En,
}

impl Lang {
    /// Resolve o idioma ativo considerando o modo configurado e a localidade do sistema.
    pub fn resolve(mode: LanguageMode) -> Self {
        match mode {
            LanguageMode::PtBr => Lang::Pt,
            LanguageMode::EnUs => Lang::En,
            LanguageMode::Auto => {
                let sys = sys_locale::get_locale().unwrap_or_default().to_lowercase();
                if sys.starts_with("pt") {
                    Lang::Pt
                } else {
                    // Padrão global para outros sistemas: Inglês
                    Lang::En
                }
            }
        }
    }

    pub fn is_en(&self) -> bool {
        matches!(self, Lang::En)
    }

    pub fn is_pt(&self) -> bool {
        matches!(self, Lang::Pt)
    }
}

/// Chaves de tradução da aplicação inteira
#[derive(Debug, Clone, Copy)]
pub enum TextKey {
    // Menu lateral / Geral
    Home,
    QuickConvert,
    Productivity,
    Calculator,
    Pdf,
    Photos,
    Video,
    Audio,
    BatchRename,
    Zip,
    ColorsPasswords,
    Queue,
    History,
    Settings,
    Installer,
    SearchPlaceholder,
    OfflineBadge,
    WelcomeTitle,
    WelcomeSubtitle,
    SystemStatus,
    StatusReady,
    RecentNews,
    News1,
    News2,
    News3,
    ToolsCenter,
    OpenFolder,
    // Configurações
    SettingsTitle,
    SettingsDesc,
    AppearanceSection,
    DarkModeLabel,
    AutoOpenOutputLabel,
    HistoryLimitLabel,
    HistoryLimitDesc,
    LanguageLabel,
    LanguageDesc,
    OpenDataFolderBtn,
    RestoreDefaultsBtn,
    PrivacySectionTitle,
    PrivacyNotice1,
    PrivacyNotice2,
    DataFolderLabel,
    // Botões comuns
    Apply,
    Cancel,
    Save,
    Export,
    Close,
    RestoreOriginal,
    // Fotos / Imagens
    PhotoStudioTitle,
    PhotoStudioDesc,
    TabDeveloper,
    TabBatch,
    TabResizeConvert,
    InputPhotoLabel,
    InputPhotoHint,
    OutputPhotoLabel,
    OutputPhotoHint,
    LivePreviewMonitor,
    LivePreviewDesc,
    CropRatioTitle,
    CropRatioDesc,
    AdjustmentsTitle,
    AdjustmentsDesc,
    SubtabLight,
    SubtabColor,
    SubtabEffects,
    SubtabPresets,
    QuickStylesTitle,
    PresetsBoxTitle,
    ExportPhotoBtn,
    BatchFolderTitle,
    BatchFolderDesc,
    BatchRunBtn,
    // Conversor Fácil
    QuickConvertTitle,
    QuickConvertDesc,
    DragDropHere,
    DropOrPickFile,
    ChooseFileBtn,
    SelectOutputFormat,
    // Vídeo
    VideoTitle,
    VideoDesc,
    // Áudio
    AudioTitle,
    AudioDesc,
    // PDF
    PdfTitle,
    PdfDesc,
    // ZIP
    ZipTitle,
    ZipDesc,
}

impl Lang {
    pub fn get(&self, key: TextKey) -> &'static str {
        match self {
            Lang::Pt => match key {
                TextKey::Home => "Início",
                TextKey::QuickConvert => "Conversor Fácil",
                TextKey::Productivity => "Produtividade",
                TextKey::Calculator => "Calculadora",
                TextKey::Pdf => "PDF",
                TextKey::Photos => "Fotos & Imagens",
                TextKey::Video => "Vídeo",
                TextKey::Audio => "Áudio",
                TextKey::BatchRename => "Renomear em Lote",
                TextKey::Zip => "Compactador ZIP",
                TextKey::ColorsPasswords => "Cores & Senhas",
                TextKey::Queue => "Fila de Operações",
                TextKey::History => "Histórico",
                TextKey::Settings => "Configurações",
                TextKey::Installer => "Instalador Setup",
                TextKey::SearchPlaceholder => "🔍 Pesquisar ferramenta…",
                TextKey::OfflineBadge => "100% Offline",
                TextKey::WelcomeTitle => "Bem-vindo ao Axolote Suite!",
                TextKey::WelcomeSubtitle => "Sua central nativa e offline de produtividade, conversão e ferramentas criativas para Windows.",
                TextKey::SystemStatus => "Status do Sistema",
                TextKey::StatusReady => "Tudo pronto e funcionando offline.",
                TextKey::RecentNews => "Últimas Novidades",
                TextKey::News1 => "• Revelador de fotos estilo Lightroom com histograma RGB e lote",
                TextKey::News2 => "• Conversor universal 1-clique sem sobrescrita silenciosa",
                TextKey::News3 => "• Suporte a multi-idioma (detecção automática)",
                TextKey::ToolsCenter => "Central de Ferramentas",
                TextKey::OpenFolder => "Abrir pasta de saída",
                TextKey::SettingsTitle => "Configurações",
                TextKey::SettingsDesc => "Personalize idioma, tema visual e comportamento do Axolote Suite. Salvo localmente.",
                TextKey::AppearanceSection => "Aparência e Idioma",
                TextKey::DarkModeLabel => "Usar tema escuro (identidade visual roxo & azul neon)",
                TextKey::AutoOpenOutputLabel => "Abrir automaticamente a pasta de saída no Explorer ao concluir uma operação",
                TextKey::HistoryLimitLabel => "Limite de registros no Histórico",
                TextKey::HistoryLimitDesc => "Define quantas operações recentes são mantidas no arquivo historico.json.",
                TextKey::LanguageLabel => "Idioma da Interface",
                TextKey::LanguageDesc => "Escolha o idioma do aplicativo ou deixe em detecção automática do Windows.",
                TextKey::OpenDataFolderBtn => "📂  Abrir pasta de dados do Axolote",
                TextKey::RestoreDefaultsBtn => "🔄  Restaurar padrões",
                TextKey::PrivacySectionTitle => "100% Offline e Privado",
                TextKey::PrivacyNotice1 => "• Nenhuma conta ou login necessário.",
                TextKey::PrivacyNotice2 => "• Zero telemetria, zero anúncios e zero envio de dados para servidores.",
                TextKey::DataFolderLabel => "• Pasta onde ficam suas configurações, notas e tarefas:",
                TextKey::Apply => "Aplicar",
                TextKey::Cancel => "Cancelar",
                TextKey::Save => "Salvar",
                TextKey::Export => "Exportar",
                TextKey::Close => "Fechar",
                TextKey::RestoreOriginal => "Restaurar original",
                TextKey::PhotoStudioTitle => "Estúdio & Revelador de Fotos",
                TextKey::PhotoStudioDesc => "Edição não destrutiva com pré-visualização ao vivo, histograma RGB, controles profissionais de Luz, Cor e Efeitos, sincronização em lote e presets de cinema.",
                TextKey::TabDeveloper => "🎨  Revelador Profissional (Ao Vivo)",
                TextKey::TabBatch => "⚡  Sincronizar em Lote (Pasta)",
                TextKey::TabResizeConvert => "📐  Redimensionar & Formatos",
                TextKey::InputPhotoLabel => "Foto ou imagem de entrada",
                TextKey::InputPhotoHint => "Formatos aceitos: PNG, JPG/JPEG, WebP, BMP, GIF, TIFF e ICO.",
                TextKey::OutputPhotoLabel => "Arquivo ou pasta de saída (opcional)",
                TextKey::OutputPhotoHint => "Se deixar em branco, a foto editada é salva ao lado da original sem sobrescrever (gravação atômica).",
                TextKey::LivePreviewMonitor => "Monitor de Pré-visualização",
                TextKey::LivePreviewDesc => "Renderização com iluminação, cor e filtros atualizados em tempo real.",
                TextKey::CropRatioTitle => "Enquadramento & Proporção",
                TextKey::CropRatioDesc => "Corte no formato exato para Instagram (1:1), YouTube (16:9), Reels/TikTok (9:16) ou 4:3.",
                TextKey::AdjustmentsTitle => "Ajustes Não Destrutivos",
                TextKey::AdjustmentsDesc => "Mude as abas para ajustar parâmetros finos ou escolher estilos.",
                TextKey::SubtabLight => "☀ Luz",
                TextKey::SubtabColor => "🌈 Cor",
                TextKey::SubtabEffects => "✨ Efeitos",
                TextKey::SubtabPresets => "🎬 Presets",
                TextKey::QuickStylesTitle => "Filtros de Estilo Rápido",
                TextKey::PresetsBoxTitle => "Looks Fotográficos Prontos (Estilo Lightroom / PhotoCraft)",
                TextKey::ExportPhotoBtn => "💾   Exportar Foto Revelada em Resolução Máxima",
                TextKey::BatchFolderTitle => "Processador em Lote (Estilo LightCraft)",
                TextKey::BatchFolderDesc => "Ajuste uma foto no Revelador para achar o tom perfeito, venha aqui e sincronize centenas de imagens de uma só vez para uma subpasta sem tocar nas originais.",
                TextKey::BatchRunBtn => "⚡   Aplicar Edições em Todas as Fotos da Pasta",
                TextKey::QuickConvertTitle => "Conversor Fácil Universal",
                TextKey::QuickConvertDesc => "Escolha qualquer arquivo do computador ou arraste para cá e converta em 1 clique sem sobrescrever o original.",
                TextKey::DragDropHere => "Solte arquivos aqui para converter",
                TextKey::DropOrPickFile => "Arraste um arquivo ou clique para escolher",
                TextKey::ChooseFileBtn => "📂  Escolher arquivo…",
                TextKey::SelectOutputFormat => "Escolha o formato de saída:",
                TextKey::VideoTitle => "Editor & Compressor de Vídeo",
                TextKey::VideoDesc => "Corte clipes, altere a velocidade, converta formatos, aplique filtros de cinema e comprima com CRF.",
                TextKey::AudioTitle => "Estúdio de Áudio",
                TextKey::AudioDesc => "Converta áudios, extraia a trilha de vídeos e ajuste bitrate com alta fidelidade.",
                TextKey::PdfTitle => "Central Completa de PDF",
                TextKey::PdfDesc => "Junte múltiplos PDFs, divida páginas e intervalos e consulte metadados técnicos.",
                TextKey::ZipTitle => "Compactador ZIP Seguro",
                TextKey::ZipDesc => "Compacte arquivos e pastas completas ou extraia com blindagem anti Zip-Slip.",
            },
            Lang::En => match key {
                TextKey::Home => "Home",
                TextKey::QuickConvert => "Quick Convert",
                TextKey::Productivity => "Productivity",
                TextKey::Calculator => "Calculator",
                TextKey::Pdf => "PDF",
                TextKey::Photos => "Photos & Images",
                TextKey::Video => "Video",
                TextKey::Audio => "Audio",
                TextKey::BatchRename => "Batch Rename",
                TextKey::Zip => "ZIP Archiver",
                TextKey::ColorsPasswords => "Colors & Passwords",
                TextKey::Queue => "Tasks Queue",
                TextKey::History => "History",
                TextKey::Settings => "Settings",
                TextKey::Installer => "Setup Installer",
                TextKey::SearchPlaceholder => "🔍 Search tool…",
                TextKey::OfflineBadge => "100% Offline",
                TextKey::WelcomeTitle => "Welcome to Axolote Suite!",
                TextKey::WelcomeSubtitle => "Your native, offline productivity, conversion, and creative workstation for Windows.",
                TextKey::SystemStatus => "System Status",
                TextKey::StatusReady => "All systems operational and running offline.",
                TextKey::RecentNews => "Recent Updates",
                TextKey::News1 => "• Lightroom-inspired Photo Developer with live RGB histogram and batch processing",
                TextKey::News2 => "• 1-Click Universal Converter with safe atomic writes",
                TextKey::News3 => "• Multi-language support with automatic OS detection",
                TextKey::ToolsCenter => "Tools Center",
                TextKey::OpenFolder => "Open output folder",
                TextKey::SettingsTitle => "Settings",
                TextKey::SettingsDesc => "Customize language, theme, and behavior. All preferences saved locally on your device.",
                TextKey::AppearanceSection => "Appearance & Language",
                TextKey::DarkModeLabel => "Use dark theme (deep purple & neon blue identity)",
                TextKey::AutoOpenOutputLabel => "Automatically open output folder in File Explorer when a task completes",
                TextKey::HistoryLimitLabel => "History entries limit",
                TextKey::HistoryLimitDesc => "Specifies how many recent operations are kept in historico.json.",
                TextKey::LanguageLabel => "Interface Language",
                TextKey::LanguageDesc => "Choose app language or let it automatically detect from your Windows OS.",
                TextKey::OpenDataFolderBtn => "📂  Open Axolote data directory",
                TextKey::RestoreDefaultsBtn => "🔄  Restore original defaults",
                TextKey::PrivacySectionTitle => "100% Offline & Private",
                TextKey::PrivacyNotice1 => "• No accounts, logins, or cloud telemetry required.",
                TextKey::PrivacyNotice2 => "• Zero ads, zero tracking, and zero data leaves your PC.",
                TextKey::DataFolderLabel => "• Local storage directory for configs, notes, and tasks:",
                TextKey::Apply => "Apply",
                TextKey::Cancel => "Cancel",
                TextKey::Save => "Save",
                TextKey::Export => "Export",
                TextKey::Close => "Close",
                TextKey::RestoreOriginal => "Restore original",
                TextKey::PhotoStudioTitle => "Photo Studio & Developer",
                TextKey::PhotoStudioDesc => "Non-destructive photo editing with real-time preview, live RGB histogram, professional Light, Color, and Effects controls, batch folder sync, and cinematic looks.",
                TextKey::TabDeveloper => "🎨  Pro Developer (Live)",
                TextKey::TabBatch => "⚡  Batch Sync (Folder)",
                TextKey::TabResizeConvert => "📐  Resize & Formats",
                TextKey::InputPhotoLabel => "Input photo or image",
                TextKey::InputPhotoHint => "Supported formats: PNG, JPG/JPEG, WebP, BMP, GIF, TIFF, and ICO.",
                TextKey::OutputPhotoLabel => "Output file or folder (optional)",
                TextKey::OutputPhotoHint => "If empty, the edited image is saved beside original with atomic write (never overwrites).",
                TextKey::LivePreviewMonitor => "Live Preview Monitor",
                TextKey::LivePreviewDesc => "Real-time rendering of lighting, color grading, and creative filters.",
                TextKey::CropRatioTitle => "Framing & Aspect Ratio",
                TextKey::CropRatioDesc => "Crop to exact proportions for Instagram (1:1), YouTube (16:9), Reels/TikTok (9:16), or 4:3.",
                TextKey::AdjustmentsTitle => "Non-destructive Controls",
                TextKey::AdjustmentsDesc => "Switch tabs to fine-tune exposure, color tone, or select pre-made styles.",
                TextKey::SubtabLight => "☀ Light",
                TextKey::SubtabColor => "🌈 Color",
                TextKey::SubtabEffects => "✨ Effects",
                TextKey::SubtabPresets => "🎬 Presets",
                TextKey::QuickStylesTitle => "Quick Style Filters",
                TextKey::PresetsBoxTitle => "Photo Looks (Lightroom / PhotoCraft Style)",
                TextKey::ExportPhotoBtn => "💾   Export Full Resolution Photo",
                TextKey::BatchFolderTitle => "Batch Folder Sync (LightCraft Style)",
                TextKey::BatchFolderDesc => "Fine-tune one photo in the Developer, then apply the exact same adjustments across hundreds of images in a folder without altering original files.",
                TextKey::BatchRunBtn => "⚡   Apply Edits to All Photos in Folder",
                TextKey::QuickConvertTitle => "Universal Quick Converter",
                TextKey::QuickConvertDesc => "Choose any file from your PC or drag & drop to convert in 1 click without silent overwriting.",
                TextKey::DragDropHere => "Drop files here to convert",
                TextKey::DropOrPickFile => "Drag and drop a file or click to browse",
                TextKey::ChooseFileBtn => "📂  Choose file…",
                TextKey::SelectOutputFormat => "Select output format:",
                TextKey::VideoTitle => "Video Editor & Compressor",
                TextKey::VideoDesc => "Trim clips on timeline, adjust playback speed, convert formats, apply cinema styles, and compress via CRF.",
                TextKey::AudioTitle => "Audio Studio",
                TextKey::AudioDesc => "Convert between audio formats, extract tracks from video, and adjust bitrate with high fidelity.",
                TextKey::PdfTitle => "Complete PDF Hub",
                TextKey::PdfDesc => "Merge multiple PDFs, split pages and intervals, and inspect document metadata.",
                TextKey::ZipTitle => "Secure ZIP Archiver",
                TextKey::ZipDesc => "Compress files or full directories and extract archives safely with Zip-Slip protection.",
            },
        }
    }
}
