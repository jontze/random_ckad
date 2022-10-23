use anyhow::{Context, Ok};
use clap::ValueEnum;
use rand::Rng;

use crate::question::{self, Question};

pub(crate) const CORE_CONCEPTS: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/a.core_concepts.md";
pub(crate) const MULTI_CONTAINER_POD: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/b.multi_container_pods.md";
pub(crate) const POD_DESIGN: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/c.pod_design.md";
pub(crate) const CONFIGURATION: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/d.configuration.md";
pub(crate) const OBSERVABILITY: &str =
    "https://github.com/dgkanatsios/CKAD-exercises/raw/main/e.observability.md";
pub(crate) const SERVICES_NETWORKING: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/f.services.md";
pub(crate) const HELM: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/h.helm.md";
pub(crate) const CRD: &str =
    "https://raw.githubusercontent.com/dgkanatsios/CKAD-exercises/main/i.crd.md";

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub(crate) enum Categories {
    Core,
    MultiContainer,
    PodDesign,
    Configuration,
    Observability,
    ServicesNetworking,
    Helm,
    Crd,
}

pub(crate) const PAGES: &[(Categories, &str)] = &[
    (Categories::Core, CORE_CONCEPTS),
    (Categories::MultiContainer, MULTI_CONTAINER_POD),
    (Categories::PodDesign, POD_DESIGN),
    (Categories::Configuration, CONFIGURATION),
    (Categories::Observability, OBSERVABILITY),
    (Categories::ServicesNetworking, SERVICES_NETWORKING),
    (Categories::Helm, HELM),
    (Categories::Crd, CRD),
];

pub(crate) struct Page {
    category: Categories,
    url: String,
}

impl Page {
    fn get_url(category: &Categories) -> Option<String> {
        PAGES
            .iter()
            .find(|(key, _)| key == category)
            .map(|(_, url)| url.to_string())
    }

    pub fn from_category(category: Categories) -> anyhow::Result<Self> {
        let url = Self::get_url(&category).context("Failed to find url")?;
        Ok(Self { category, url })
    }

    pub fn from_random() -> anyhow::Result<Self> {
        let mut rng = rand::thread_rng();
        let rand_pick = PAGES
            .get(rng.gen_range(0..PAGES.len()))
            .map(|(cat, _)| cat)
            .context("Failed to get a random page")?;
        Self::from_category(rand_pick.to_owned())
    }

    pub async fn fetch_questions(&self) -> anyhow::Result<Vec<Question>> {
        let questions_md = reqwest::get(&self.url).await?.text().await?;
        let questions = question::parse_questions(questions_md, self.category);
        Ok(questions)
    }

    pub async fn fetch_random_question(&self) -> anyhow::Result<Question> {
        let questions = self.fetch_questions().await?;
        let ran_question = question::get_random_question(&questions)
            .context("Failed to extract random question")?;
        Ok(ran_question.to_owned())
    }
}
