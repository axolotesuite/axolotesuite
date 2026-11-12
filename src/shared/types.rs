//! Tipos compartilhados entre os módulos.

use std::path::PathBuf;

use crate::shared::i18n::Lang;
use crate::shared::theme::{self, Colors};

/// Resultado de uma operação executada por um serviço.
#[derive(Debug, Clone)]
pub struct OperationResult {
    pub success: bool,
    /// Frase curta mostrada em destaque (ex.: "3 arquivos extraídos").
    pub title: String,
    /// Linhas extras de detalhe (ex.: lista de arquivos, avisos).
    pub details: Vec<String>,
    /// Arquivo ou pasta gerado(a), para o botão "Abrir pasta".
    pub output: Option<PathBuf>,
}

impl OperationResult {
    pub fn ok(title: impl Into<String>) -> Self {
        Self {
            success: true,
            title: title.into(),
            details: Vec::new(),
            output: None,
        }
    }

    pub fn err(title: impl Into<String>) -> Self {
        Self {
            success: false,
            title: title.into(),
            details: Vec::new(),
            output: None,
        }
    }

    pub fn with_details(mut self, details: Vec<String>) -> Self {
        self.details = details;
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.details.push(detail.into());
        self
    }

    pub fn with_output(mut self, output: impl Into<PathBuf>) -> Self {
        self.output = Some(output.into());
        self
    }
}

/// Informações de ambiente para montar as telas de forma responsiva.
#[derive(Debug, Clone, Copy)]
pub struct Env {
    /// Largura disponível para o conteúdo (sem a barra lateral).
    pub width: f32,
    /// Tema escuro ativo?
    pub dark: bool,
    /// Idioma ativo resolvido (Pt ou En).
    pub lang: Lang,
}

impl Env {
    pub fn t(&self, key: crate::shared::i18n::TextKey) -> &'static str {
        self.lang.get(key)
    }

    pub fn c(&self) -> Colors {
        if self.dark {
            theme::dark()
        } else {
            theme::light()
        }
    }

    /// Tela estreita (celular / janela pequena): tudo em uma coluna.
    pub fn narrow(&self) -> bool {
        self.width < 680.0
    }

    /// Tela média (organizada em 2 colunas equilibradas).
    pub fn is_medium(&self) -> bool {
        self.width >= 680.0 && self.width < 1040.0
    }

    /// Tela ampla (desktop completo em Bento Grid).
    pub fn is_desktop_wide(&self) -> bool {
        self.width >= 1040.0
    }

    /// Tela larga o bastante para duas colunas lado a lado.
    pub fn wide(&self) -> bool {
        self.width >= 980.0
    }

    /// Quantas colunas cabem numa grade cujos itens têm `min_item` px.
    pub fn columns(&self, min_item: f32, max: usize) -> usize {
        let usable = (self.width - 48.0).max(min_item);
        ((usable / min_item).floor() as usize).clamp(1, max)
    }

    /// Espaçamento interno da página conforme o tamanho da tela.
    pub fn page_padding(&self) -> u16 {
        if self.narrow() {
            14
        } else if self.wide() {
            32
        } else {
            22
        }
    }
}
