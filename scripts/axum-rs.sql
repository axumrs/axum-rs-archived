CREATE TABLE subject (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    summary VARCHAR(255) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(slug)
);

CREATE INDEX idx_subject_slug ON subject (slug);

CREATE TABLE topic (
    id BIGSERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    subject_id INTEGER NOT NULL REFERENCES subject(id),
    self_slug VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    summary VARCHAR(255) NOT NULL,
    author VARCHAR(50) NOT NULL,
    hit INTEGER NOT NULL DEFAULT 0,
    dateline INTEGER NOT NULL DEFAULT 0,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(slug)
);

CREATE INDEX idx_topic_slug ON topic (slug);

CREATE TABLE topic_content (
    topic_id BIGINT NOT NULL PRIMARY KEY REFERENCES topic(id),
    md VARCHAR NOT NULL,
    html VARCHAR NOT NULL
);

CREATE TABLE tag (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(name)
);

CREATE INDEX idx_tag_name ON tag (name);

CREATE TABLE topic_tag (
    topic_id BIGINT NOT NULL REFERENCES topic(id),
    tag_id INT NOT NULL REFERENCES tag(id),
    is_del BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(topic_id,tag_id)
);
