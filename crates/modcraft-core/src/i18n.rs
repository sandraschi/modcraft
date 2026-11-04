//! ModCraft Internationalization (i18n) Engine.
//!
//! Follows the fleet standard: typed locale enums, zero-panic string lookups,
//! multi-language translations (EN, DE, FR, ES, JA, KO, ZH),
//! and modular computational geometry / topology terminology localization.
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unimplemented)]
#![deny(clippy::todo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AppLocale {
    #[default]
    En, // English (Global Default)
    De, // German (Bauhaus modular design native)
    Fr, // French (Le Corbusier Modulor native)
    Es, // Spanish
    Ja, // Japanese (Metabolism architecture native)
    Ko, // Korean
    Zh, // Chinese
}

impl AppLocale {
    pub fn all() -> &'static [AppLocale] {
        &[
            AppLocale::En,
            AppLocale::De,
            AppLocale::Fr,
            AppLocale::Es,
            AppLocale::Ja,
            AppLocale::Ko,
            AppLocale::Zh,
        ]
    }

    pub fn code(&self) -> &'static str {
        match self {
            AppLocale::En => "en",
            AppLocale::De => "de",
            AppLocale::Fr => "fr",
            AppLocale::Es => "es",
            AppLocale::Ja => "ja",
            AppLocale::Ko => "ko",
            AppLocale::Zh => "zh",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            AppLocale::En => "English (US/UK)",
            AppLocale::De => "Deutsch (DE/AT/CH)",
            AppLocale::Fr => "Français (FR)",
            AppLocale::Es => "Español (ES/LATAM)",
            AppLocale::Ja => "日本語 (JA)",
            AppLocale::Ko => "한국어 (KO)",
            AppLocale::Zh => "中文 (ZH)",
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            AppLocale::En => "🇬🇧",
            AppLocale::De => "🇩🇪",
            AppLocale::Fr => "🇫🇷",
            AppLocale::Es => "🇪🇸",
            AppLocale::Ja => "🇯🇵",
            AppLocale::Ko => "🇰🇷",
            AppLocale::Zh => "🇨🇳",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum I18nKey {
    // Spatial & Geometry
    VoxelGrid,
    ParametricPrimitives,
    BooleanUnion,
    BooleanDifference,
    BooleanIntersection,
    MeshTopology,
    ExportObjStl,
    StatusReady,
}

pub fn t(key: I18nKey, locale: AppLocale) -> &'static str {
    match (key, locale) {
        (I18nKey::VoxelGrid, AppLocale::En) => "Voxel Space Grid",
        (I18nKey::VoxelGrid, AppLocale::De) => "Voxel-Raumgitter",
        (I18nKey::VoxelGrid, AppLocale::Fr) => "Grille spatiale de voxels",
        (I18nKey::VoxelGrid, AppLocale::Es) => "Rejilla espacial de vóxeles",
        (I18nKey::VoxelGrid, AppLocale::Ja) => "ボクセル空間グリッド",
        (I18nKey::VoxelGrid, AppLocale::Ko) => "복셀 공간 격자",
        (I18nKey::VoxelGrid, AppLocale::Zh) => "体素空间三维网格",

        (I18nKey::ParametricPrimitives, AppLocale::En) => "Parametric Solid Primitives",
        (I18nKey::ParametricPrimitives, AppLocale::De) => "Parametrische Grundkörper",
        (I18nKey::ParametricPrimitives, AppLocale::Fr) => "Primitives solides paramétriques",
        (I18nKey::ParametricPrimitives, AppLocale::Es) => "Primitivas sólidas paramétricas",
        (I18nKey::ParametricPrimitives, AppLocale::Ja) => "パラメトリック基本立体 (プリミティブ)",
        (I18nKey::ParametricPrimitives, AppLocale::Ko) => "파라메트릭 솔리드 기본도형",
        (I18nKey::ParametricPrimitives, AppLocale::Zh) => "参数化实体几何基元",

        (I18nKey::BooleanUnion, AppLocale::En) => "Boolean Union (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::De) => "Boolesche Vereinigung (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::Fr) => "Union booléenne (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::Es) => "Unión booleana (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::Ja) => "ブーリアン結合・和集合 (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::Ko) => "불리언 합집합 (A ∪ B)",
        (I18nKey::BooleanUnion, AppLocale::Zh) => "布尔并集求和 (A ∪ B)",

        (I18nKey::BooleanDifference, AppLocale::En) => "Boolean Difference (A - B)",
        (I18nKey::BooleanDifference, AppLocale::De) => "Boolesche Differenz (A - B)",
        (I18nKey::BooleanDifference, AppLocale::Fr) => "Différence booléenne (A - B)",
        (I18nKey::BooleanDifference, AppLocale::Es) => "Diferencia booleana (A - B)",
        (I18nKey::BooleanDifference, AppLocale::Ja) => "ブーリアン差分・削り取り (A - B)",
        (I18nKey::BooleanDifference, AppLocale::Ko) => "불리언 차집합 (A - B)",
        (I18nKey::BooleanDifference, AppLocale::Zh) => "布尔差集切除 (A - B)",

        (I18nKey::BooleanIntersection, AppLocale::En) => "Boolean Intersection (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::De) => "Boolescher Schnitt (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::Fr) => "Intersection booléenne (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::Es) => "Intersección booleana (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::Ja) => "ブーリアン交差・共通部分 (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::Ko) => "불리언 교집합 (A ∩ B)",
        (I18nKey::BooleanIntersection, AppLocale::Zh) => "布尔交集保留 (A ∩ B)",

        (I18nKey::MeshTopology, AppLocale::En) => "Manifold Mesh Topology",
        (I18nKey::MeshTopology, AppLocale::De) => "Mannigfaltige Netztopologie",
        (I18nKey::MeshTopology, AppLocale::Fr) => "Topologie de maillage",
        (I18nKey::MeshTopology, AppLocale::Es) => "Topología de malla",
        (I18nKey::MeshTopology, AppLocale::Ja) => "閉多様体メッシュトポロジー",
        (I18nKey::MeshTopology, AppLocale::Ko) => "매니폴드 메시 토폴로지",
        (I18nKey::MeshTopology, AppLocale::Zh) => "流形网格拓扑校验",

        (I18nKey::ExportObjStl, AppLocale::En) => "Export Mesh (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::De) => "Netz exportieren (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::Fr) => "Exporter le maillage (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::Es) => "Exportar malla (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::Ja) => "3Dメッシュ出力 (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::Ko) => "3D 메시 내보내기 (OBJ / STL / GLTF)",
        (I18nKey::ExportObjStl, AppLocale::Zh) => "导出 3D 网格模型 (OBJ / STL / GLTF)",

        (I18nKey::StatusReady, AppLocale::En) => "ModCraft Modular Geometry Kernel Ready",
        (I18nKey::StatusReady, AppLocale::De) => "ModCraft Modular-Geometrie-Kernel bereit",
        (I18nKey::StatusReady, AppLocale::Fr) => "Noyau de géométrie modulaire ModCraft prêt",
        (I18nKey::StatusReady, AppLocale::Es) => "Núcleo de geometría modular ModCraft listo",
        (I18nKey::StatusReady, AppLocale::Ja) => "ModCraft モジュラー幾何学演算コア 準備完了",
        (I18nKey::StatusReady, AppLocale::Ko) => "ModCraft 모듈러 기하학 커널 준비완료",
        (I18nKey::StatusReady, AppLocale::Zh) => "ModCraft 模块化计算几何内核就绪",
    }
}
