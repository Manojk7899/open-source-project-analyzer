"use client";

import { FormEvent, useState } from "react";

type Repository = {
  database_id: number;
  full_name: string;
  description: string | null;
  stars: number;
  forks: number;
  open_issues: number;
  language: string | null;
  license: string | null;
  default_branch: string;
  url: string;
};

type DeterministicScore = {
  activity_score: number;
  documentation_score: number;
  community_score: number;
  maintenance_score: number;
  overall_score: number;
};

type AiAnalysis = {
  summary: string;
  documentation_quality_score: number;
  onboarding_score: number;
  beginner_friendliness_score: number;
  strengths: string[];
  weaknesses: string[];
  suggested_improvements: string[];
  model: string;
};

type Report = {
  repository: Repository;
  deterministic_score: DeterministicScore;
  ai_analysis: AiAnalysis;
  final_score: number;
};

type FetchResponse = {
  database_id: number;
};

const API_URL =
    process.env.NEXT_PUBLIC_API_URL ??
  "http://127.0.0.1:8081";

const steps = [
  "GitHub",
  "Scoring",
  "Gemini",
  "Report",
];

async function request<T>(
    path: string,
    options?: RequestInit,
): Promise<T> {
  const response = await fetch(`${API_URL}${path}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
    },
  });

  const responseText = await response.text();
  let body: { error?: string } & T;

  try {
    body = JSON.parse(responseText) as { error?: string } & T;
  } catch {
    throw new Error(
        responseText ||
        `Request failed with status ${response.status}`,
    );
  }

  if (!response.ok) {
    throw new Error(
        body.error ??
        `Request failed with status ${response.status}`,
    );
  }

  return body as T;
}

export default function Home() {
  const [url, setUrl] = useState("");
  const [report, setReport] = useState<Report | null>(
      null,
  );
  const [currentStep, setCurrentStep] = useState(-1);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState("");

  async function analyzeRepository(
      event: FormEvent<HTMLFormElement>,
  ) {
    event.preventDefault();

    setIsLoading(true);
    setError("");
    setReport(null);

    try {
      setCurrentStep(0);

      const repository = await request<FetchResponse>(
          "/api/repository/fetch",
          {
            method: "POST",
            body: JSON.stringify({ url }),
          },
      );

      const id = repository.database_id;

      setCurrentStep(1);

      await request(`/api/repository/${id}/score`, {
        method: "POST",
      });

      setCurrentStep(2);

      await request(
          `/api/repository/${id}/ai-analysis`,
          {
            method: "POST",
          },
      );

      setCurrentStep(3);

      const finalReport = await request<Report>(
          `/api/repository/${id}/report`,
      );

      setReport(finalReport);
      setCurrentStep(steps.length);
    } catch (caughtError) {
      setError(
          caughtError instanceof Error
              ? caughtError.message
              : "Unexpected error",
      );

      setCurrentStep(-1);
    } finally {
      setIsLoading(false);
    }
  }

  return (
      <main className="page">
        <section className="hero">
          <p className="eyebrow">
            Rust + Axum + PostgreSQL + Gemini
          </p>

          <h1>Open-source project analyzer</h1>

          <p className="subtitle">
            Analyze a GitHub repository using real metrics,
            deterministic scoring and structured AI analysis.
          </p>

          <form
              className="analyze-form"
              onSubmit={analyzeRepository}
          >
            <label htmlFor="repository-url">
              GitHub repository URL
            </label>

            <div className="form-row">
              <input
                  id="repository-url"
                  type="url"
                  placeholder="https://github.com/owner/repository"
                  value={url}
                  disabled={isLoading}
                  onChange={(event) =>
                      setUrl(event.target.value)
                  }
                  required
              />

              <button disabled={isLoading} type="submit">
                {isLoading && <span className="spinner" />}

                {isLoading
                    ? "Analyzing..."
                    : "Analyze repository"}
              </button>
            </div>
          </form>

          {(isLoading || currentStep === steps.length) && (
              <div className="progress">
                <strong>
                  {currentStep === steps.length
                      ? "Analysis complete"
                      : "Analyzing repository"}
                </strong>

                <div className="steps">
                  {steps.map((step, index) => {
                    const complete =
                        currentStep === steps.length ||
                        index < currentStep;

                    const active = index === currentStep;

                    return (
                        <div
                            className={`step ${
                                complete ? "complete" : ""
                            } ${active ? "active" : ""}`}
                            key={step}
                        >
                    <span>
                      {complete ? "✓" : index + 1}
                    </span>

                          {step}
                        </div>
                    );
                  })}
                </div>
              </div>
          )}

          {error && (
              <div className="error-message">{error}</div>
          )}
        </section>

        {report && (
            <section className="report">
              <header className="report-header">
                <div>
                  <p className="eyebrow">
                    Analysis complete
                  </p>

                  <h2>
                    <a
                        href={report.repository.url}
                        target="_blank"
                        rel="noreferrer"
                    >
                      {report.repository.full_name}
                    </a>
                  </h2>

                  <p>
                    {report.repository.description ??
                        "No description available."}
                  </p>
                </div>

                <div className="final-score">
                  <strong>{report.final_score}</strong>
                  <span>Final score</span>
                </div>
              </header>

              <div className="metrics">
                <Metric
                    label="Stars"
                    value={report.repository.stars.toLocaleString()}
                />
                <Metric
                    label="Forks"
                    value={report.repository.forks.toLocaleString()}
                />
                <Metric
                    label="Open issues"
                    value={report.repository.open_issues.toLocaleString()}
                />
                <Metric
                    label="Language"
                    value={report.repository.language ?? "Unknown"}
                />
                <Metric
                    label="License"
                    value={report.repository.license ?? "None"}
                />
                <Metric
                    label="Default branch"
                    value={report.repository.default_branch}
                />
              </div>

              <section className="panel">
                <div className="panel-heading">
                  <div>
                    <p className="eyebrow">
                      GitHub metrics
                    </p>
                    <h3>Deterministic score</h3>
                  </div>

                  <strong className="panel-score">
                    {
                      report.deterministic_score
                          .overall_score
                    }
                  </strong>
                </div>

                <div className="score-grid">
                  <Score
                      label="Activity"
                      value={
                        report.deterministic_score
                            .activity_score
                      }
                  />
                  <Score
                      label="Documentation"
                      value={
                        report.deterministic_score
                            .documentation_score
                      }
                  />
                  <Score
                      label="Community"
                      value={
                        report.deterministic_score
                            .community_score
                      }
                  />
                  <Score
                      label="Maintenance"
                      value={
                        report.deterministic_score
                            .maintenance_score
                      }
                  />
                </div>
              </section>

              <section className="panel">
                <div className="panel-heading">
                  <div>
                    <p className="eyebrow">
                      Semantic review
                    </p>
                    <h3>Gemini analysis</h3>
                  </div>

                  <span className="model">
                {report.ai_analysis.model}
              </span>
                </div>

                <p className="summary">
                  {report.ai_analysis.summary}
                </p>

                <div className="score-grid">
                  <Score
                      label="Documentation quality"
                      value={
                        report.ai_analysis
                            .documentation_quality_score
                      }
                  />
                  <Score
                      label="Onboarding"
                      value={
                        report.ai_analysis.onboarding_score
                      }
                  />
                  <Score
                      label="Beginner friendliness"
                      value={
                        report.ai_analysis
                            .beginner_friendliness_score
                      }
                  />
                </div>
              </section>

              <div className="insights">
                <Insight
                    title="Strengths"
                    items={report.ai_analysis.strengths}
                />
                <Insight
                    title="Weaknesses"
                    items={report.ai_analysis.weaknesses}
                />
                <Insight
                    title="Suggested improvements"
                    items={
                      report.ai_analysis
                          .suggested_improvements
                    }
                />
              </div>
            </section>
        )}
      </main>
  );
}

function Metric({
                  label,
                  value,
                }: {
  label: string;
  value: string;
}) {
  return (
      <div className="metric">
        <span>{label}</span>
        <strong>{value}</strong>
      </div>
  );
}

function Score({
                 label,
                 value,
               }: {
  label: string;
  value: number;
}) {
  return (
      <div className="score">
        <div>
          <span>{label}</span>
          <strong>{value}</strong>
        </div>

        <div className="score-track">
          <div
              className="score-fill"
              style={{ width: `${value}%` }}
          />
        </div>
      </div>
  );
}

function Insight({
                   title,
                   items,
                 }: {
  title: string;
  items: string[];
}) {
  return (
      <section className="panel insight">
        <h3>{title}</h3>

        <ul>
          {items.map((item) => (
              <li key={item}>{item}</li>
          ))}
        </ul>
      </section>
  );
}