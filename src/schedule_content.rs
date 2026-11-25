use crate::programs::CURRICULUM;
use leptos::prelude::*;

fn grade_label(grade: &str) -> String {
    if grade == "K" {
        "Kindergarten".to_string()
    } else {
        format!("Grade {grade}")
    }
}

#[component]
pub fn Curriculum() -> impl IntoView {
    view! {
        <section id="programs-by-grade" class="guide-section" aria-labelledby="grade-heading">
            <div class="guide-section-heading">
                <h2 id="grade-heading">"Programs by Grade"</h2>
                <p>"The grade levels below are suggested starting points."</p>
            </div>
            <div class="grade-table-wrap">
                <table class="grade-table" role="table" aria-labelledby="grade-heading" aria-describedby="grade-table-caption">
                    <caption id="grade-table-caption">"Choose one program per show. The running time applies to the selected program."</caption>
                    <thead role="rowgroup">
                        <tr role="row"><th scope="col" role="columnheader">"Suggested Grade"</th><th scope="col" role="columnheader">"Available Programs"</th><th scope="col" role="columnheader">"Running Time"</th></tr>
                    </thead>
                    <tbody role="rowgroup">
                        {CURRICULUM.into_iter().map(|(grade, titles, duration)| view! {
                            <tr role="row">
                                <th scope="row" role="rowheader">{grade_label(grade)}</th>
                                <td role="cell"><ul class="program-choices" role="list">
                                    {titles.lines().map(|title| view! { <li>{title}</li> }).collect_view()}
                                </ul></td>
                                <td role="cell" class="grade-time">{duration}</td>
                            </tr>
                        }).collect_view()}
                    </tbody>
                </table>
            </div>
            <p class="grade-guidance">"Shows can be adapted for other grades and multi-age audiences. Schools can select different programs as students progress through the years, allowing annual visits to introduce fresh material."</p>
        </section>
    }
}

const ONE_DAY: [usize; 3] = [2, 3, 4];
const DAY_1: [usize; 3] = [0, 2, 4];
const DAY_2: [usize; 3] = [1, 3, 5];
const LUNCH_CHANGEOVER: &str =
    "Lunch can serve as the changeover when it allows at least 15 minutes between shows.";

struct VisitEntry {
    grade: Option<String>,
    title: &'static str,
    duration: Option<&'static str>,
}

fn performance(grade_index: usize) -> VisitEntry {
    let (grade, titles, duration) = CURRICULUM[grade_index];
    // Each sample visit features the first listed choice for its grade.
    VisitEntry {
        grade: Some(grade_label(grade)),
        title: titles.lines().next().unwrap_or(titles),
        duration: Some(duration),
    }
}

fn visit_entries([opening, before_lunch, after_lunch]: [usize; 3]) -> [VisitEntry; 5] {
    [
        performance(opening),
        VisitEntry {
            grade: None,
            title: "Changeover",
            duration: Some("at least 15 minutes"),
        },
        performance(before_lunch),
        VisitEntry {
            grade: None,
            title: "Lunch",
            duration: None,
        },
        performance(after_lunch),
    ]
}

fn itinerary_text(grades: [usize; 3]) -> String {
    visit_entries(grades)
        .into_iter()
        .map(|entry| {
            [entry.grade.as_deref(), Some(entry.title), entry.duration]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .join(" · ")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn one_day_example_text() -> String {
    format!(
        "Example One-Day Visit\n\n{}\n\n{LUNCH_CHANGEOVER}",
        itinerary_text(ONE_DAY)
    )
}

pub fn two_day_example_text() -> String {
    format!(
        "Example Two-Day Visit\n\nDay 1\n\n{}\n\nDay 2\n\n{}\n\n{LUNCH_CHANGEOVER}",
        itinerary_text(DAY_1),
        itinerary_text(DAY_2)
    )
}

#[component]
fn Itinerary(grades: [usize; 3]) -> impl IntoView {
    view! {
        <ol class="itinerary" role="list">
            {visit_entries(grades).into_iter().map(|entry| {
                let class = if entry.grade.is_some() { "itinerary-show" } else { "itinerary-break" };
                view! {
                    <li class=class>
                        {entry.grade.map(|grade| view! { <span class="itinerary-grade">{grade}</span>" " })}
                        <strong class="itinerary-title">{entry.title}</strong>
                        {entry.duration.map(|duration| view! { " "<span class="itinerary-time">{duration}</span> })}
                    </li>
                }
            }).collect_view()}
        </ol>
    }
}

#[component]
pub fn VisitSchedules() -> impl IntoView {
    view! {
        <section id="visit-schedules" class="guide-section" aria-labelledby="schedule-heading">
            <div class="guide-section-heading">
                <h2 id="schedule-heading">"Plan a Visit Around Your School Day"</h2>
                <p>"Once your date or dates are confirmed, the day’s schedule can be arranged around your school’s routine."</p>
            </div>
            <ul class="schedule-rules" role="list">
                <li>"Allow "<strong>"at least 15 minutes between shows"</strong>" for audience changeover and resetting supplies and materials."</li>
                <li>"Neal arrives "<strong>"45–60 minutes before the first show."</strong></li>
            </ul>
            <p class="schedule-note">"The schedules below are examples. Final arrangements are coordinated for each visit. "{LUNCH_CHANGEOVER}</p>
            <section class="visit-example" aria-labelledby="one-day-heading">
                <h3 id="one-day-heading">"Example One-Day Visit"</h3>
                <Itinerary grades=ONE_DAY/>
            </section>
            <section class="visit-example" aria-labelledby="two-day-heading">
                <h3 id="two-day-heading">"Example Two-Day Visit"</h3>
                <div class="visit-days">
                    <section class="visit-day" aria-labelledby="day-1-heading">
                        <h4 id="day-1-heading">"Day 1"</h4>
                        <Itinerary grades=DAY_1/>
                    </section>
                    <section class="visit-day" aria-labelledby="day-2-heading">
                        <h4 id="day-2-heading">"Day 2"</h4>
                        <Itinerary grades=DAY_2/>
                    </section>
                </div>
            </section>
        </section>
    }
}
