CREATE TABLE repository_scores (
                                   id BIGSERIAL PRIMARY KEY,
                                   repository_id BIGINT NOT NULL UNIQUE
                                       REFERENCES repositories(id)
                                           ON DELETE CASCADE,
                                   activity_score INTEGER NOT NULL,
                                   documentation_score INTEGER NOT NULL,
                                   community_score INTEGER NOT NULL,
                                   maintenance_score INTEGER NOT NULL,
                                   overall_score INTEGER NOT NULL,
                                   reasons TEXT[] NOT NULL DEFAULT '{}',
                                   calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);