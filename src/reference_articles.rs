// Verified original summaries. Sources and limits: docs/ARTICLE-RESEARCH-2026-11-08.md.
const ARTICLES: &[Article] = &[
    Article {
        publisher: "The Provincetown Independent",
        title: "For Illustrator Neal Nichols Jr., Drawing Is Like Breathing",
        credit: "Susannah Elisabeth Fulcher | March 31, 2021",
        paragraphs: &[
            "Susannah Elisabeth Fulcher’s profile follows Neal from his childhood on Cape Cod to a career built around drawing, observation, and travel. It describes an artist fascinated by perspective and the textures of old buildings, whose talent was recognized by teachers and a gallery while he was still young.",
            "The story traces Geography Gameshow to an encounter with a teacher who saw Neal sketch a map of Europe from memory. That classroom invitation became a way to combine illustration with learning about the world. The article reports school performances across the United States, Europe, Australia, New Zealand, and Japan. It also explores his community work in Alaska and summer drawing classes in Wellfleet, connecting his international teaching with the coastal landscapes and local relationships that continue to inspire his art.",
        ],
    },
    Article {
        publisher: "The Provincetown Independent",
        title: "A Life That’s Everywhere and All Right Here",
        credit: "Dorothea Samaha | August 27, 2025",
        paragraphs: &[
            "Dorothea Samaha’s profile visits Neal’s Eastham studio, where maps, travel journals, drawings, and souvenirs document decades of exploration. A detailed United States map he made as a teenager offers an early glimpse of the observation and memory behind his work. During the interview, he demonstrates how quickly a country or the world can take shape beneath his marker.",
            "The article presents mapmaking and illustration as complementary ways of remembering places. Drawings record individual scenes; maps connect those scenes to a larger journey. It follows the experiences that have informed his work, from Cape Cod and cross-country driving to travels overseas. Geography Gameshow brings that combination into schools through live map drawing and historical storytelling. The profile also notes his long-running summer art classes with Wellfleet Recreation, showing how a life of travel remains connected to a home community.",
        ],
    },
    Article {
        publisher: "The Nome Nugget",
        title: "Quick-draw delivers the world to Nome students",
        credit: "Tyler Rhodes | September 23, 2010 | Page 7",
        paragraphs: &[
            "Tyler Rhodes reports on Neal’s return to Nome Elementary after a tour of Bering Strait School District communities. Starting with blank paper, Neal develops a map while introducing places, dates, and clues that help students remember them. Students then become contestants, answering questions for small prizes collected during his travels.",
            "The report describes three energetic presentations, with younger pupils exploring Alaska and fifth and sixth graders tackling the Lower 48. It also records a week of art teaching in Stebbins before the school tour. The central focus is the connection between drawing and attention: a map emerging in real time gives students something to follow, discuss, and recall together.",
            "Additional coverage · November 21, 2013: Classroom photo captions document another Geography Gameshow visit to Nome Elementary. They describe students answering geography questions, including one about the major lakes of the American Midwest, and receiving prizes. This is photo coverage rather than a second feature article.",
        ],
    },
    Article {
        publisher: "Overdrive",
        title: "The World At His Fingertips",
        credit: "Kristen Buck / Overdrive | December 13, 2004",
        paragraphs: &[
            "This early magazine feature explores how Neal combined life on the road with a growing classroom program. It describes him drawing the world from memory and adding details in response to students’ questions and the topics they were studying. Geography becomes a shared guessing game, with travel souvenirs helping make distant places tangible.",
            "The article connects that approach to a lifelong interest in illustration and to journeys across the United States and overseas. Transporting fine art and later working for a Cape Cod moving company gave him opportunities to observe places closely while continuing to draw. By 2004, the show was taking him into classrooms around the country, with frequent visits to Alaska. The profile captures the early development of a program shaped by curiosity, artistic practice, and direct interaction with students.",
        ],
    },
    Article {
        publisher: "University of Delaware",
        title: "University assists with Delaware GIS Day field trip",
        credit: "University of Delaware | November 24, 2009",
        paragraphs: &[
            "The university reports on a Delaware GIS Day field trip that brought approximately 260 fifth graders to the Dover Air Mobility Command Museum on November 18, 2009. Geography Gameshow was part of a larger day of practical exploration during Geography Awareness Week.",
            "Neal drew the United States from memory, adding historical and cultural information and inviting students to answer questions as the map developed. The report notes students’ surprise at how much they could recall through the activity. Other stations introduced geographic information systems, aerial imagery, compasses, and GPS, alongside demonstrations of how geography is used in working life. The account places Neal’s drawing-based teaching within a broader educational event supported by university, government, and community volunteers.",
            "The Delaware Geological Survey republished this university report on November 25, 2009.",
        ],
    },
    Article {
        publisher: "Peacehaven Community School",
        title: "Geography Gameshow",
        credit: "Peacehaven Community School | December 4 visit (year not given)",
        paragraphs: &[
            "Peacehaven’s school report describes a return visit for Year 7 students. Neal begins by drawing the world from memory, using the developing map to introduce the history and variety of different countries. The school presents this as a way to build geographical understanding and encourage curiosity about learning.",
            "Students then test what they have remembered through several games and a final challenge, with some receiving prizes. The report also describes a smaller geography lesson in which students worked directly with Neal to develop their skills. Its closing invitation encourages families to look at world maps and consider places they might one day visit. The report identifies the visit as Thursday, December 4; a year is not established.",
        ],
    },
    Article {
        publisher: "James Hargest College",
        title: "Year 10 Social Studies — The Geography Gameshow",
        credit: "James Hargest College — May 10, 2024",
        paragraphs: &[
            "A Year 10 student account describes a world-map presentation at James Hargest College in New Zealand. Neal began with ten questions that the class could answer only in part. As he drew the world from memory, he introduced important locations and offered ways to remember them, including major rivers, mountain ranges, lakes, and oceans.",
            "The account emphasizes both the speed and accuracy of his drawing and the amount students remembered by the end. A closing question round gave them a chance to apply what they had heard, followed by a return to the original ten questions. The student describes the session as an opportunity to learn from an enthusiastic traveler whose knowledge and drawing worked together throughout the lesson.",
        ],
    },
    Article {
        publisher: "Peacehaven Community School",
        title: "The Geography Gameshow. Newsletter, Edition 4",
        credit: "Peacehaven Community School — December 16, 2022",
        paragraphs: &[
            "Peacehaven’s newsletter records Neal’s return on December 1, 2022, when all Year 7 students watched him construct a world map from memory. Alongside facts about different places, the report highlights his emphasis on practice, engagement, and students taking an active role in their own learning.",
            "The school describes the question-and-answer stage as an essential part of the experience. Students could use what they had just learned to compete for objects from Neal’s travels. The account connects the impressive drawing with a practical educational purpose: encouraging pupils to pay attention, remember information, and put that knowledge to use. It is a separate visit from the school’s undated December 4 report.",
        ],
    },
    Article {
        publisher: "Lancing Prep Hove",
        title: "The Lower Quad — Geography. Page 24",
        credit: "Lancing Prep Hove — 2016–2017 annual review",
        paragraphs: &[
            "The school’s annual review places Geography Gameshow among a year of classroom learning, visiting specialists, and geographical fieldwork. Years 6 and 7 attended the show at the end of the autumn term. The report links its location knowledge to the Common Entrance syllabus, with Neal drawing Europe and the world from memory while adding stories and games. It describes an interactive format supported by prizes gathered during his travels. The account shows how the performance was used as part of a wider geography program rather than as an isolated entertainment event.",
        ],
    },
    Article {
        publisher: "Lancing Prep Hove",
        title: "Geography Gameshow returns for Years 7 and 8",
        credit: "Lancing Prep Hove — December 2022",
        paragraphs: &[
            "This newsletter records Neal’s return from Cape Cod after a pandemic-related interruption to his visits. Year 7 explored Europe, while Year 8 tested its knowledge of the world. The school describes detailed maps drawn from memory and games designed to help pupils retain geographical information. Objects from Neal’s travels provided prizes and a connection to the places being discussed. The report highlights the combination of art, travel, and audience participation, noting that pupils and staff enjoyed an experience intended to strengthen their knowledge of global locations.",
        ],
    },
    Article {
        publisher: "Lancing Prep Hove",
        title: "Geography Game Show. Newsletter, page 11",
        credit: "Lancing Prep Hove — December 20, 2024",
        paragraphs: &[
            "Lancing Prep Hove’s end-of-term newsletter describes another visit for Years 7 and 8. The younger year group took part in the Europe show, and the older group explored the world. Neal’s maps, drawn from memory, provided the setting for questions and activities that helped students remember geographical information. The report connects his artistic ability with his experience of travel and describes the incentive of prizes brought back from different places. Presented as a geography-themed end-of-term event, the visit offered pupils and staff a lively way to develop their knowledge of where places are.",
            "Other archive records include the school’s 2017–2018 annual review and its December 15, 2023 newsletter. The latter describes Europe and world-map shows for Years 7 and 8. Together, these publications document several return visits.",
        ],
    },
    Article {
        publisher: "Mrs. Thomas’s Fourth Grade",
        title: "Neil “The Geography Guy”",
        credit: "Mrs. Thomas’s Fourth Grade — November 7, 2013",
        paragraphs: &[
            "Mrs. Thomas’s classroom account describes an artist’s visit to McCarthy School to teach United States geography through drawing. As Neal developed a map, students learned about state capitals and dates, historical figures, landforms, rivers, and natural resources. The post records how the drawing became a way to connect locations with information about the people and landscapes associated with them. Published on November 7, 2013, it provides a teacher’s brief account of the lesson. The source uses the spelling “Neil Nichols.”",
        ],
    },
    Article {
        publisher: "Mrs. Begin’s Happy Campers",
        title: "A Very Busy December!",
        credit: "Mrs. Begin’s Happy Campers — January 5, 2016",
        paragraphs: &[
            "In a January 5, 2016 recap of the preceding December, Mrs. Begin includes Neal Nichols Jr.’s Geography Game Show among the second graders’ classroom highlights. The teacher describes the presentation positively. This is a brief mention within a wider account of school activities, rather than a detailed review of the show.",
        ],
    },
    Article {
        publisher: "Mike Mahaffie",
        title: "Such a Busy Week",
        credit: "Mike Mahaffie — November 20, 2009",
        paragraphs: &[
            "Writing after Geography Awareness Week, Delaware GIS coordinator Mike Mahaffie reflects on volunteering at a geography field day for fifth graders at the Air Mobility Command Museum. He singles out Neal’s Geography Game Show as a successful addition to that year’s event. His account describes a large United States wall map drawn by hand and used as a playful teaching tool. The post offers a participant’s perspective on the day, distinct from the university’s formal report. It places the show alongside a wider effort by geography professionals to share their knowledge with schoolchildren.",
        ],
    },
];
