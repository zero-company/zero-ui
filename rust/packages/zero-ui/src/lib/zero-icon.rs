#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ZeroIcon {
    #[default]
    ZeroLogo,
}

const ZERO_LOGO: icondata_core::IconData = icondata_core::IconData {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: r###"<circle cx="11" cy="11" r="8" />
<line x1="21" y1="21" x2="16.65" y2="16.65" />"###,
};

impl From<ZeroIcon> for icondata_core::IconData {
    fn from(icon: ZeroIcon) -> icondata_core::IconData {
        match icon {
            ZeroIcon::ZeroLogo => ZERO_LOGO,
        }
    }
}

