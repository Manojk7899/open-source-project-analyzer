CREATE TABLE repository_ai_analyses (
                                        id BIGSERIAL PRIMARY KEY,
                                        repository_id BIGINT NOT NULL UNIQUE
                                            REFERENCES repositories(id)
                                                ON DELETE CASCADE,
                                        summary TEXT NOT NULL,
                                        documentation_quality_score INTEGER NOT NULL,
                                        onboarding_score INTEGER NOT NULL,
                                        beginner_friendliness_score INTEGER NOT NULL,
                                        strengths TEXT[] NOT NULL DEFAULT '{}',
                                        weaknesses TEXT[] NOT NULL DEFAULT '{}',
                                        suggested_improvements TEXT[] NOT NULL DEFAULT '{}',
                                        model TEXT NOT NULL,
                                        analyzed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);