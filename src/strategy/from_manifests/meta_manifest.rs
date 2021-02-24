use crate::channel::Channel;
use crate::{RustReleasesError, TResult};

#[derive(Debug)]
pub(in crate::strategy::from_manifests) struct MetaManifest {
    manifests: Vec<ManifestSource>,
}

impl MetaManifest {
    pub(in crate::strategy::from_manifests) fn try_from_str<T: AsRef<str>>(
        item: T,
    ) -> TResult<Self> {
        let item = item.as_ref();

        let manifests = item
            .lines()
            .map(ManifestSource::try_from_str)
            .collect::<TResult<Vec<_>>>()?;

        Ok(Self { manifests })
    }

    pub(in crate::strategy::from_manifests) fn manifests(&self) -> &[ManifestSource] {
        &self.manifests
    }
}

#[derive(Debug)]
pub(in crate::strategy::from_manifests) struct ManifestSource {
    url: String,
    channel: Channel,
    date: String,
}

impl ManifestSource {
    pub(in crate::strategy::from_manifests) fn try_from_str<T: AsRef<str>>(
        item: T,
    ) -> TResult<Self> {
        let item = item.as_ref();

        let channel = Self::parse_channel(item)?;
        let date = Self::parse_date(item)?;

        Ok(Self {
            url: format!("https://{}", item),
            channel,
            date,
        })
    }

    pub(in crate::strategy::from_manifests) fn url(&self) -> &str {
        &self.url
    }

    pub(in crate::strategy::from_manifests) fn channel(&self) -> Channel {
        self.channel
    }

    pub(in crate::strategy::from_manifests) fn date(&self) -> &str {
        &self.date
    }

    fn parse_date(input: &str) -> TResult<String> {
        // an input has the following form:
        // `static.rust-lang.org/dist/YYYY-MM-DD/channel-rust-CHANNEL.toml`
        // where YYYY    is the year,
        //   and MM      is the month,
        //   and DD      is the day,
        //   and CHANNEL is one of 'stable' | 'beta' | 'nightly'

        let date = input
            .get(26..36)
            .ok_or(RustReleasesError::ParseManifestDate)?;
        Ok(date.to_string())
    }

    fn parse_channel(input: &str) -> TResult<Channel> {
        Ok(if input.contains("beta") {
            Channel::Beta
        } else if input.contains("nightly") {
            Channel::Nightly
        } else if input.contains("stable") {
            Channel::Stable
        } else {
            return Err(RustReleasesError::ParseManifestSource);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strategy::from_manifests::DocumentSource;

    #[test]
    fn test_parse_meta_manifest() {
        let path = [env!("CARGO_MANIFEST_DIR"), "/resources/manifests.txt"].join("");
        let meta_file = DocumentSource::LocalPath(path.into());

        let buffer = meta_file.load().unwrap();
        let meta_manifest = MetaManifest::try_from_str(String::from_utf8(buffer).unwrap());
        assert!(meta_manifest.is_ok());
    }
}
