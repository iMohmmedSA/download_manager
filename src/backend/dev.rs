use ratatui::widgets::Row;

use crate::frontend::download_list_state::ListState;

#[derive(Debug, Clone)]
pub struct DevList {
    pub name: String,
    pub url: String,
    pub size: String,
}

impl DevList {
    pub fn new(name: String, url: String, size: String) -> Self {
        DevList { name, url, size }
    }

    pub fn list() -> ListState<DevList> {
        let mut list = ListState::new();

        list.items = vec![
            DevList::new("Parallel".to_string(), "".to_string(), "".to_string()),
            DevList::new(
                "ubuntu-24.04.2-desktop-amd64.iso".to_string(),
                "https://releases.ubuntu.com/24.04.2/ubuntu-24.04.2-desktop-amd64.iso".to_string(),
                "5.9 GiB".to_string(),
            ),
            DevList::new(
                "LibreOffice_25.2.3_Linux_x86-64_deb.tar.gz".to_string(),
                "https://download.documentfoundation.org/libreoffice/stable/25.2.3/deb/x86_64/LibreOffice_25.2.3_Linux_x86-64_deb.tar.gz".to_string(),
                "198.9 MiB".to_string(),
            ),
            DevList::new(
                "10Gb.dat".to_string(),
                "https://proof.ovh.net/files/10Gb.dat".to_string(),
                "10 GiB".to_string(),
            ),
            DevList::new(
                "100Mb.dat".to_string(),
                "https://proof.ovh.net/files/100Mb.dat".to_string(),
                "100 MiB".to_string(),
            ),
            DevList::new(
                "100MB.zip".to_string(),
                "http://speedtest.tele2.net/100MB.zip".to_string(),
                "100 MiB".to_string(),
            ),
            DevList::new(
                "10Mb.dat".to_string(),
                "https://proof.ovh.net/files/10Mb.dat".to_string(),
                "10 MiB".to_string(),
            ),
            DevList::new(
                "10Mb.dat".to_string(),
                "https://proof.ovh.net/files/10Mb.dat".to_string(),
                "10 MiB".to_string(),
            ),

            DevList::new("Sequential".to_string(), "".to_string(), "".to_string()),
            DevList::new(
                "mozilla B2G_2_5_20160125_MERGEDAY.zip".to_string(),
                "https://github.com/mozilla/gecko-dev/archive/refs/tags/B2G_2_5_20160125_MERGEDAY.zip".to_string(),
                "316.9 MiB".to_string(),
            ),
            DevList::new(
                "tensorflow v2.18.1.zip".to_string(),
                "https://github.com/tensorflow/tensorflow/archive/refs/tags/v2.18.1.zip".to_string(),
                "90.8 MiB".to_string(),
            ),
            DevList::new(
                "opencv 4.11.0.zip".to_string(),
                "https://github.com/opencv/opencv/archive/refs/tags/4.11.0.zip".to_string(),
                "88.9 MiB".to_string(),
            ),

        ];

        list
    }
}

impl From<&DevList> for Row<'_> {
    fn from(item: &DevList) -> Self {
        let item = item.clone();
        Row::new(vec![item.name, item.size, item.url])
    }
}
