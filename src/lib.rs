//! # CFDI.rs
//!
//! Crate para Deserializar un XML de un CFDI en structs nativos en Rust.
//!
//!
//! ## Comprobante
//!
//! El struct principal es [`Comprobante`]. Al igual que en un xml de un CFDI, es el nodo de origen.
//!
//! Tambien hay structs para otros subnodos del xml, incluyendo sus atributos. Estos incluyen
//! ['Emisor'], ['Receptor'] y ['TimbreFiscalDigital']
//!
//! Este crate por el momento solo contempla la estructura más básica de un CFDI.
//! No se intenta generar un modelo completo del estandar de los CFDI 4.0+
//! Hasta esta versión, solo se implementa el siguient modelo:
//!
//! ```markdown
//!|-Comprobante
//!     |-- Emisor
//!          |**-> rfc
//!          |**-> nombre
//!          |**-> regimen_fiscal
//!     |-- Receptor
//!          |**-> rfc
//!          |**-> nombre
//!          |**-> regimen_fiscal
//!          |**-> uso_cfdi
//!     |-- Conceptos
//!     |-- Complemento (opcional) - Incluye TimbreFiscalDigital
//!     |**-> total
//!     |**-> subtotal
//!     |**-> fecha
//!     |**-> forma_de_pago
//!     |**-> descuento
//!     |**-> tipo_comprobante
//!     |**-> forma_de_pago
//!
//!
//! |-- Representan subnodos
//! |**-> representan atributos del nodo
//! ```
//!
//!
//! Sin embargo, cualquier persona que necesite más detalle puede revisar el codigo fuente y
//! de ahi basarse para agregar los subnodos que necesite. Realmente no es dificil, pero es
//! un poco tedioso. Si agregan subnodos les agradecería hicieran un Pull Request.
//!
//!
//!
//!
//!
//! #### Ejemplo:
//! Usando [`Comprobante`]:
//! ```rust
//! use cfdi::{DatosPrincipales, get_datos_principales};
//!
//! let path = ("path/to/file.xml");
//! let xml_string = std::fs::read_to_string(&path).unwrap();
//!
//!
//! if let Ok(parsed) = parse_cfdi(&xml_string) {
//!
//!         println!("Emisor: {}, RFC: {}", parsed.emisor.nombre, parsed.emisor.rfc);
//!         println!("Receptor: {}, RFC: {}", parsed.receptor.nombre, parsed.receptor.rfc);
//!         println!("Subtotal:  {}", parsed.subtotal);
//!         println!("Total:  {}", parsed.total);
//!         println!("Fecha Factura:  {}", parsed.fecha);
//!         println!("UUID:  {}", parsed.complemento);
//!         println!("Fecha Timbrado:  {}", parsed.fecha_timbrado);
//!
//!
//!         // Otros datos no incluidos en [`DatosPrincipales`]:
//!         match &parsed.complemento {
//!             Some(c) => c.timbre_fiscal_digital
//!                 .clone()
//!                 .map(|tfd| println!("Certificado SAT: {}", tfd.no_certificado_sat),
//!                 None => None,
//!        }
//! }
//!```
//!
//!
//! ## Datos Principales
//! [`DatosPrincipales`] es un struct que facilita recopilar en 1 solo nivel los principales
//! atributos del cfdi. Asume que los datos se encuntran correctamente definidos en el cfdi.
//! Además, no incluye otros datos que a veces ocupan mucho espacio, o simplemente no conviene
//! estar manejando (ej. Sello, certificado, etc.)
//!
//!
//!
//! ```rust
//! use cfdi::{DatosPrincipales, get_datos_principales};
//!
//! let path = ("path/to/file.xml");
//! let xml_string = std::fs::read_to_string(&path).unwrap();
//!
//! if let Ok(parsed) = parse_cfdi(&xml_string) {
//!         let datos: DatosPrincipales = parsed.get_datos_principales();
//!
//!         println!("Emisor: {}, RFC: {}", datos.emisor_nombre, datos.emisor_rfc);
//!         println!("Receptor: {}, RFC: {}", datos.receptor_nombre, datos.receptor_rfc);
//!         println!("Subtotal:  {}", datos.subtotal);
//!         println!("Total:  {}", datos.total);
//!         println!("Fecha Factura:  {}", datos.fecha);
//!         println!("UUID:  {}", datos.get_uuid());
//!         println!("Fecha Timbrado:  {}", datos.get_fecha_timbrado());
//!
//! }
//! ```

use anyhow::Result;
use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

/// Nodo principal del CFDI. De aqui se pueden obtener todos los demás subnodos.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Comprobante {
    // #[serde(rename = "InformacionGlobal")]
    //pub informacion_global: InformacionGlobal,
    //
    // #[serde(rename = "CfdiRelacionados")]
    // pub cfdi_relacionados: CfdiRelacionados,
    /// Total de la factura
    #[serde(rename = "@Total")]
    pub total: f32,

    /// Subtotal de la factura
    #[serde(rename = "@SubTotal")]
    pub subtotal: f32,

    /// Fecha de la factura
    #[serde(rename = "@Fecha")]
    pub fecha: String,

    /// Forma de pago
    #[serde(rename = "@FormaPago")]
    pub forma_pago: Option<String>,

    //TODO: Enum para Forma de Pago
    /// Descuento de la factura
    #[serde(rename = "@Descuento")]
    pub descuento: Option<String>,

    /// Tipo de comprobante:
    #[serde(rename = "@TipoDeComprobante")]
    pub tipo_comprobante: String,

    #[serde(rename = "Emisor")]
    pub emisor: Emisor,
    #[serde(rename = "Receptor")]
    pub receptor: Receptor,

    #[serde(rename = "Conceptos")]
    pub conceptos: Conceptos,

    #[serde(rename = "Complemento")]
    pub complemento: Option<Complemento>,
}

/// Información del Contribuyente Emisor del Complemento
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Emisor {
    /// RFC del emisor del comprobante
    #[serde(rename = "@Rfc")]
    pub rfc: String,

    /// Nombre, Denominación o Razón Social del emisor del comprobante
    #[serde(rename = "@Nombre")]
    pub nombre: String,

    /// Clave del Régimen del Emisor -- Ver Catalogos en SAT
    #[serde(rename = "@RegimenFiscal")]
    pub regimen_fiscal: String,
    //TODO: agregar `FacAtrAdquiriente`
}

/// Información del Contribuyente Receptor del Complemento
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Receptor {
    /// RFC del receptor del comprobante
    #[serde(rename = "@Rfc")]
    pub rfc: String,

    /// Nombre, Denominación o Razón Social del receptor del comprobante
    #[serde(rename = "@Nombre")]
    pub nombre: String,

    /// Clave del Régimen del Emisor -- Ver Catálogos en SAT
    #[serde(rename = "@RegimenFiscalReceptor")]
    pub regimen_fiscal: String,

    /// Clave del uso que el receptor dará a este CFDI -- Ver Catálogos en SAT
    #[serde(rename = "@UsoCFDI")]
    pub uso_cfdi: String,
    // TODO:: Agregar `ResidenciaFiscal` y `NumRegIdTrib`
}

/// Representa un concepto de la factura.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Concepto {
    #[serde(rename = "@ClaveProdServ")]
    pub clave_product: String,

    #[serde(rename = "@Cantidad")]
    pub cantidad: f32,

    #[serde(rename = "@ClaveUnidad")]
    pub clave_unidad: String,

    #[serde(rename = "@Unidad")]
    pub unidad: Option<String>,

    #[serde(rename = "@Descripcion")]
    pub descripcion: String,

    #[serde(rename = "@ValorUnitario")]
    pub valor_unitario: String,

    #[serde(rename = "@Importe")]
    pub importe: f32,

    #[serde(rename = "@Descuento")]
    pub descuento: Option<f32>,
}

/// Comlemento de la factura. Incluye Timbre Fiscal (si se encuentra)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Complemento {
    #[serde(rename = "TimbreFiscalDigital")]
    pub timbre_fiscal_digital: Option<TimbreFiscalDigital>,
}

/// Representa el Timbre Fiscal, incluye el UUID, certificado SAT, etc.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimbreFiscalDigital {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "@UUID")]
    pub uuid: String,
    #[serde(rename = "@FechaTimbrado")]
    pub fecha_timbrado: String,

    #[serde(rename = "@NoCertificadoSAT")]
    pub no_certificado_sat: String,
}

/// Lista de [`Concepto`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Conceptos {
    #[serde(rename = "Concepto")]
    pub concepto: Vec<Concepto>,
}

/// Intenta generar un objeto de tipo `Comprobante` a partir de un texto (&str)
pub fn parse_cfdi(xml_content: &str) -> Result<Comprobante> {
    let res: Comprobante = from_str(xml_content)?;
    Ok(res)
}

/// Utility Struct - para guardar datos principales de un comprobante en 1 solo struct
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatosPrincipales {
    pub total: f32,
    pub subtotal: f32,
    pub fecha: String,
    pub emisor_nombre: String,
    pub emisor_rfc: String,
    pub receptor_nombre: String,
    pub receptor_rfc: String,
    pub uuid: Option<String>,
    pub fecha_timbrado: Option<String>,
    pub conceptos: Vec<Concepto>,
}

impl Comprobante {
    /// Regresa un vector con los [`Concepto`] de la Factura
    pub fn get_conceptos(&self) -> Vec<Concepto> {
        self.conceptos.concepto.clone()
    }

    /// Regresa un `Option<String>`, con el UUID dentro del Some si la factura tiene Complemento
    pub fn get_uuid(&self) -> Option<String> {
        match &self.complemento {
            Some(c) => c.timbre_fiscal_digital.clone().map(|tfd| tfd.uuid),
            None => None,
        }
    }

    /// Regresa un `Option<String>`, con la fecha de timbrado dentro del Some si la factura tiene Complemento
    pub fn get_fecha_timbrado(&self) -> Option<String> {
        match &self.complemento {
            Some(c) => c
                .timbre_fiscal_digital
                .clone()
                .map(|tfd| tfd.fecha_timbrado),
            None => None,
        }
    }

    /// Genera un `DatosPrincipales` con los datos del comprobante
    pub fn get_datos_principales(self) -> DatosPrincipales {
        let total = self.total;
        let subtotal = self.subtotal;
        let fecha = self.fecha.clone();
        let emisor_nombre = self.emisor.nombre.clone();
        let emisor_rfc = self.emisor.rfc.clone();
        let receptor_nombre = self.receptor.nombre.clone();
        let receptor_rfc = self.receptor.rfc.clone();
        let uuid = self.get_uuid();
        let fecha_timbrado = self.get_fecha_timbrado();
        let conceptos = self.get_conceptos();

        DatosPrincipales {
            total,
            subtotal,
            fecha,
            emisor_nombre,
            emisor_rfc,
            receptor_nombre,
            receptor_rfc,
            uuid,
            fecha_timbrado,
            conceptos,
        }
    }
}
