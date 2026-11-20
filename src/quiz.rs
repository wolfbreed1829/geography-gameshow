use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Question {
    prompt: &'static str,
    choices: [&'static str; 3],
    answer: usize,
    explanation: &'static str,
}

const QUESTIONS: [[Question; 3]; 2] = [
    [
        Question {
            prompt: "Which ocean is the largest on Earth?",
            choices: ["The Atlantic", "The Pacific", "The Indian"],
            answer: 1,
            explanation: "The Pacific is the largest ocean. It stretches between the Americas and Asia and Australia.",
        },
        Question {
            prompt: "Which continent is the coldest?",
            choices: ["Antarctica", "Europe", "North America"],
            answer: 0,
            explanation: "Antarctica is the coldest continent. Even its summer temperatures can stay well below freezing.",
        },
        Question {
            prompt: "On which continent would you find the Amazon River?",
            choices: ["Africa", "Asia", "South America"],
            answer: 2,
            explanation: "The Amazon flows across South America, through one of the world’s great rainforest regions.",
        },
    ],
    [
        Question {
            prompt: "Which technique can make a flat drawing look three-dimensional?",
            choices: ["Folding the paper", "Shading", "Signing your name"],
            answer: 1,
            explanation: "Shading uses light and dark values to suggest volume, form, and where the light is coming from.",
        },
        Question {
            prompt: "What do artists call the line where land and sky seem to meet?",
            choices: ["The horizon", "The outline", "The border"],
            answer: 0,
            explanation: "The horizon line helps establish eye level and is an important guide in perspective drawing.",
        },
        Question {
            prompt: "In traditional paint mixing, blue and yellow usually make…",
            choices: ["Orange", "Purple", "Green"],
            answer: 2,
            explanation: "Blue and yellow pigments usually mix to make green. Different pigments can create very different greens!",
        },
    ],
];

#[component]
pub fn Quiz() -> impl IntoView {
    let topic = RwSignal::new(0usize);
    let step = RwSignal::new(0usize);
    let answer = RwSignal::new(None::<usize>);
    let score = RwSignal::new(0usize);
    let finished = RwSignal::new(false);
    let about = RwSignal::new(false);
    let reset = move || {
        step.set(0);
        answer.set(None);
        score.set(0);
        finished.set(false);
    };
    view! {<section class="screen-page wrap">
        <div class="screen-heading"><h1>"Challenge"</h1><button type="button" on:click=move |_|about.update(|v|*v=!*v)>{move ||if about.get(){"Play"}else{"About"}}</button></div>
        <Show when=move ||about.get() fallback=move ||view!{
            <div class="quiz-box contained-quiz">
                <div class="quiz-topic" role="group" aria-label="Challenge subject">{["Geography","Art"].into_iter().enumerate().map(|(i,label)|view!{<button type="button" aria-pressed=move ||(topic.get()==i).to_string() on:click=move |_|{topic.set(i);reset();}>{label}</button>}).collect_view()}</div>
                <Show when=move ||!finished.get() fallback=move ||view!{
                    <div class="quiz-result"><h2>{move ||format!("{} out of 3. Keep wondering.",score.get())}</h2><p>"Every question is the start of another discovery. Try a different subject, or take another trip through this one."</p><button type="button" class="button" on:click=move |_|reset()>"Play again"</button></div>
                }>
                    <p class="quiz-progress">{move ||format!("Question {} / 3",step.get()+1)}</p>
                    <h2 class="quiz-question">{move ||QUESTIONS[topic.get()][step.get()].prompt}</h2>
                    <Show when=move ||answer.get().is_none() fallback=move ||view!{
                        <div class="quiz-feedback" aria-live="polite"><strong>{move ||if answer.get()==Some(QUESTIONS[topic.get()][step.get()].answer){"You’ve got it!"}else{"A new thing to know:"}}</strong>
                            <p>{move ||QUESTIONS[topic.get()][step.get()].explanation}</p>
                            <button type="button" on:click=move |_|{if step.get()<2{step.update(|s|*s+=1);answer.set(None);}else{finished.set(true);}}>{move ||if step.get()<2{"Next question"}else{"See my result"}}</button>
                        </div>
                    }>
                        <div class="quiz-answers">{(0..3).map(|i|view!{
                            <button type="button" on:click=move |_|{if answer.get().is_none(){answer.set(Some(i));if QUESTIONS[topic.get()][step.get()].answer==i{score.update(|s|*s+=1);}}}>{move ||QUESTIONS[topic.get()][step.get()].choices[i]}</button>
                        }).collect_view()}</div>
                        <p class="quiz-hint">"Go on, take a guess. That’s how discovery starts."</p>
                    </Show>
                </Show>
            </div>
        }>
            <crate::screen::Reader text="Curiosity looks good on you.\n\nStep into the spirit of the show. Pick a world, take three questions, and see what you discover.\n\nA small taste of discovery. Not a school assessment.".to_string()/>
        </Show>
    </section>}
}
