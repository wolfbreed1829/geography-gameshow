use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Question {
    prompt: &'static str,
    choices: [&'static str; 3],
    answer: usize,
    explanation: &'static str,
}

const QUESTIONS: [[Question; 3]; 3] = [
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
            prompt: "What do plants use as an energy source to make their food?",
            choices: ["Sunlight", "Wind", "Sound"],
            answer: 0,
            explanation: "Plants capture light energy through photosynthesis to make sugars from water and carbon dioxide.",
        },
        Question {
            prompt: "What is the basic unit of a living thing?",
            choices: ["A pebble", "A cell", "A crystal"],
            answer: 1,
            explanation: "Cells are the basic units of life. Some organisms have just one; others have trillions.",
        },
        Question {
            prompt: "What do bees often carry from flower to flower?",
            choices: ["Seeds", "Soil", "Pollen"],
            answer: 2,
            explanation: "Bees transfer pollen as they visit flowers, helping many flowering plants reproduce.",
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
    let topic = RwSignal::new(0_usize);
    let step = RwSignal::new(0_usize);
    let answer = RwSignal::new(None::<usize>);
    let score = RwSignal::new(0_usize);
    let finished = RwSignal::new(false);
    let reset = move || {
        step.set(0);
        answer.set(None);
        score.set(0);
        finished.set(false);
    };
    view! {
        <section class="challenge wrap section-space" id="challenge" aria-labelledby="challenge-title">
            <div class="challenge-copy"><p class="eyebrow">"YOUR TURN / A LITTLE BRAIN SPARK"</p><h2 id="challenge-title">"Curiosity looks"<br/>"good on you."</h2><p>"Step into the spirit of the show. Pick a world, take three questions, and see what you discover."</p><div class="challenge-doodle" aria-hidden="true"><span>"?"</span><i>"✳"</i><svg viewBox="0 0 180 100"><path d="M8 25C140-10 10 90 150 68m-17-17 20 18-25 10"/></svg></div><span class="quiz-caption">"A small taste of discovery. Not a school assessment."</span></div>
            <div class="quiz-box"><div class="quiz-top"><span>"THE CURIOSITY CHALLENGE"</span><span>"✳"</span></div><div class="quiz-topic" role="group" aria-label="Challenge subject">{["Geography", "Biology", "Art"].into_iter().enumerate().map(|(i,label)| view! { <button aria-pressed=move || (topic.get() == i).to_string() class:active=move || topic.get() == i on:click=move |_| {topic.set(i); reset();}>{label}</button> }).collect_view()}</div>
            <Show when=move || !finished.get() fallback=move || view! {
                <div class="quiz-result" role="status"><span class="result-star">"✳"</span><p class="eyebrow">"THAT’S THE SPIRIT!"</p><h3>{move || format!("{} out of 3.\nKeep wondering.",score.get())}</h3><p>"Every question is the start of another discovery. Try a different subject, or take another trip through this one."</p><button class="button" on:click=move |_| reset()>"Play again"<span>"↻"</span></button></div>
            }>
                <div class="quiz-progress"><span>{move || format!("QUESTION 0{} / 03",step.get()+1)}</span><div>{(0..3).map(|i| view! {<i class:filled={move || step.get() >= i}></i>}).collect_view()}</div></div>
                <h3 class="quiz-question">{move || QUESTIONS[topic.get()][step.get()].prompt}</h3>
                <div class="quiz-answers">{(0..3).map(|i| view! {
                    <button disabled=move || answer.get().is_some() class:correct=move || answer.get().is_some() && QUESTIONS[topic.get()][step.get()].answer == i class:incorrect=move || answer.get() == Some(i) && QUESTIONS[topic.get()][step.get()].answer != i on:click=move |_| {if answer.get().is_none() {answer.set(Some(i)); if QUESTIONS[topic.get()][step.get()].answer == i {score.update(|s| *s += 1);}}}><span>{["A", "B", "C"][i]}</span>{move || QUESTIONS[topic.get()][step.get()].choices[i]}<b>{move || if answer.get().is_some() && QUESTIONS[topic.get()][step.get()].answer == i {"✓"} else if answer.get() == Some(i) {"×"} else {"↗"}}</b></button>
                }).collect_view()}</div>
                <div class="quiz-feedback" aria-live="polite"><Show when=move || answer.get().is_some()><p><b>{move || if answer.get() == Some(QUESTIONS[topic.get()][step.get()].answer) { "You’ve got it! " } else { "A new thing to know: " }}</b>{move || QUESTIONS[topic.get()][step.get()].explanation}</p><button class="text-link" on:click=move |_| {if step.get() < 2 {step.update(|s| *s += 1); answer.set(None);} else {finished.set(true);}}>{move || if step.get() < 2 {"Next question"} else {"See my result"}}<span>"→"</span></button></Show><Show when=move || answer.get().is_none()><p class="quiz-hint">"Go on, take a guess. That’s how discovery starts."</p></Show></div>
            </Show></div>
        </section>
    }
}
