use clap::ValueEnum;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorKind {
    Module,
    Entity,
    Dto,
    Service,
    Repository,
    Controller,
    Handler,
    Middleware,
    Guard,
    Validator,
    Test,
    Migration,
    Job,
    Event,
    EventHandler,
    Resource,
    #[value(name = "docker")]
    Docker,
    #[value(name = "compose")]
    Compose,
    #[value(name = "ci-github")]
    CiGithub,
    #[value(name = "ci-gitlab")]
    CiGitlab,
    #[value(name = "release")]
    Release,
}

impl GeneratorKind {
    pub fn parse(value: &str) -> Option<Self> {
        match value.to_lowercase().as_str() {
            "module" => Some(Self::Module),
            "entity" | "model" => Some(Self::Entity),
            "dto" => Some(Self::Dto),
            "service" => Some(Self::Service),
            "repository" => Some(Self::Repository),
            "controller" => Some(Self::Controller),
            "handler" => Some(Self::Handler),
            "middleware" => Some(Self::Middleware),
            "guard" => Some(Self::Guard),
            "validator" => Some(Self::Validator),
            "test" => Some(Self::Test),
            "migration" => Some(Self::Migration),
            "job" => Some(Self::Job),
            "event" => Some(Self::Event),
            "event-handler" | "eventhandler" => Some(Self::EventHandler),
            "resource" => Some(Self::Resource),
            "docker" => Some(Self::Docker),
            "compose" => Some(Self::Compose),
            "ci-github" | "ci_github" => Some(Self::CiGithub),
            "ci-gitlab" | "ci_gitlab" => Some(Self::CiGitlab),
            "release" => Some(Self::Release),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Module => "module",
            Self::Entity => "entity",
            Self::Dto => "dto",
            Self::Service => "service",
            Self::Repository => "repository",
            Self::Controller => "controller",
            Self::Handler => "handler",
            Self::Middleware => "middleware",
            Self::Guard => "guard",
            Self::Validator => "validator",
            Self::Test => "test",
            Self::Migration => "migration",
            Self::Job => "job",
            Self::Event => "event",
            Self::EventHandler => "event-handler",
            Self::Resource => "resource",
            Self::Docker => "docker",
            Self::Compose => "compose",
            Self::CiGithub => "ci-github",
            Self::CiGitlab => "ci-gitlab",
            Self::Release => "release",
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Module,
            Self::Entity,
            Self::Dto,
            Self::Service,
            Self::Repository,
            Self::Controller,
            Self::Handler,
            Self::Middleware,
            Self::Guard,
            Self::Validator,
            Self::Test,
            Self::Migration,
            Self::Job,
            Self::Event,
            Self::EventHandler,
            Self::Resource,
            Self::Docker,
            Self::Compose,
            Self::CiGithub,
            Self::CiGitlab,
            Self::Release,
        ]
    }
}

impl fmt::Display for GeneratorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
