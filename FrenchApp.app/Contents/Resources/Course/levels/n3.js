const N3_UNITS = [
  {
    id: "n3u01", level: 3,
    title: "TEF Canada decoded",
    titleFr: "Le TEF Canada décrypté",
    icon: "🎯", color: "#E63946",
    objectives: [
      "Name the 4 sections of the TEF Canada: timing, question counts, and tasks",
      "Know the widely-published score ranges that equal CLB 7 in each section",
      "Apply exam-day rules and a timing strategy for every section"
    ],
    imagePrompt: "flat vector illustration of an exam hall desk with a timer, headphones, an answer sheet and a small Canadian flag, clean minimal style, red and blue accents",
    vocab: [
      { fr: "l'épreuve (f.)", en: "a test section / exam task", tip: "One épreuve = one of the four parts of the test", audio: "n3u01-v01" },
      { fr: "le QCM", en: "multiple-choice questionnaire", tip: "QCM = Questionnaire à Choix Multiples", audio: "n3u01-v02" },
      { fr: "la consigne", en: "the instructions", tip: "Always read la consigne before you start", audio: "n3u01-v03" },
      { fr: "le brouillon", en: "rough draft", tip: "Plan on le brouillon before your final text", audio: "n3u01-v04" },
      { fr: "le barème", en: "the scoring scale", audio: "n3u01-v05" },
      { fr: "le NCLC", en: "the Canadian French-level scale", tip: "Niveaux de compétence linguistique canadiens", audio: "n3u01-v06" },
      { fr: "le chronomètre", en: "timer / stopwatch", audio: "n3u01-v07" },
      { fr: "la feuille de réponses", en: "the answer sheet", audio: "n3u01-v08" },
      { fr: "l'examinateur / l'examinatrice", en: "the examiner", audio: "n3u01-v09" },
      { fr: "le piège", en: "a trap (a distractor answer)", tip: "TEF loves les pièges — answers that sound right but aren't", audio: "n3u01-v10" }
    ],
    phrases: [
      { fr: "Puis-je aller aux toilettes, s'il vous plaît ?", en: "May I use the restroom, please?", audio: "n3u01-p01" },
      { fr: "Je n'ai pas bien compris la consigne.", en: "I didn't quite understand the instructions.", audio: "n3u01-p02" },
      { fr: "À quelle heure commence l'épreuve ?", en: "What time does the test section start?", audio: "n3u01-p03" },
      { fr: "Tout d'abord,", en: "First of all,", audio: "n3u01-p04" },
      { fr: "De plus,", en: "Furthermore,", audio: "n3u01-p05" },
      { fr: "Cependant,", en: "However,", audio: "n3u01-p06" },
      { fr: "En conclusion,", en: "In conclusion,", audio: "n3u01-p07" }
    ],
    pronunciation: {
      title: "Exam-day tips: listening & speaking",
      points: [
        "In compréhension orale you hear each recording exactly ONCE — be focused from the first second.",
        "In expression orale, speak loudly and clearly into the recorder. Mumbling costs you points.",
        "If you stumble while speaking, keep going. Never stop and restart a sentence."
      ]
    },
    grammar: {
      title: "How TEF Canada is scored",
      explanation: "The test has four independent sections, each with its own score. Listening and reading are machine-scored multiple-choice tests. Speaking and writing are each marked independently by two trained examiners. Your scores are converted to CLB / NCLC levels — the scale Canada's immigration system uses. For Express Entry, the key threshold is CLB 7 in EVERY section: that is what unlocks up to 50 bonus CRS points for French.",
      examples: [
        { fr: "J'ai obtenu 260 en compréhension orale.", en: "I scored 260 in listening.", audio: "n3u01-g01" },
        { fr: "Il faut le niveau NCLC 7 dans les quatre épreuves.", en: "You need NCLC level 7 in all four test sections.", audio: "n3u01-g02" },
        { fr: "Les résultats sont valables deux ans.", en: "The results are valid for two years.", audio: "n3u01-g03" }
      ]
    },
    dialogue: {
      title: "At the test center",
      lines: [
        { speaker: "A", fr: "Bonjour, je viens pour le TEF Canada.", en: "Hello, I'm here for the TEF Canada.", audio: "n3u01-d01" },
        { speaker: "B", fr: "Bonjour ! Vous avez une pièce d'identité ?", en: "Hello! Do you have an ID?", audio: "n3u01-d02" },
        { speaker: "A", fr: "Oui, voici mon passeport. À quelle heure commence la première épreuve ?", en: "Yes, here's my passport. What time does the first section start?", audio: "n3u01-d03" },
        { speaker: "B", fr: "À neuf heures. Les téléphones sont interdits dans la salle.", en: "At nine o'clock. Phones are forbidden in the room.", audio: "n3u01-d04" },
        { speaker: "A", fr: "Et l'expression orale, elle est enregistrée ?", en: "And the speaking test — is it recorded?", audio: "n3u01-d05" },
        { speaker: "B", fr: "Oui, elle est enregistrée et évaluée par deux examinateurs.", en: "Yes, it's recorded and assessed by two examiners.", audio: "n3u01-d06" },
        { speaker: "A", fr: "Quand est-ce que je recevrai mes résultats ?", en: "When will I receive my results?", audio: "n3u01-d07" },
        { speaker: "B", fr: "Dans deux à quatre semaines, par courriel.", en: "In two to four weeks, by email.", audio: "n3u01-d08" }
      ]
    },
    quiz: [
      { type: "choice", audio: "n3u01-q01", q: "How long is the compréhension orale section, and how many questions does it have?", choices: ["40 minutes, 60 questions", "60 minutes, 50 questions", "30 minutes, 40 questions", "15 minutes, 2 tasks"], answer: 0, explain: "CO = 40 minutes for 60 questions — about 40 seconds per question." },
      { type: "choice", audio: "n3u01-q02", q: "What is the widely-published TEF Canada score range for CLB 7 in compréhension écrite (reading)?", choices: ["207–232", "249–279", "310–348", "400–450"], answer: 0, explain: "Reading CLB 7 ≈ 207–232. Listening is 249–279; speaking and writing are 310–348. Always confirm current equivalences with your test center." },
      { type: "choice", audio: "n3u01-q03", q: "For Express Entry, what does reaching NCLC 7 in all four French abilities give you?", choices: ["Up to 50 bonus CRS points", "Automatic permanent residence", "A guaranteed job offer", "Nothing — only English counts"], answer: 0, explain: "Strong French (NCLC 7+ in all four abilities) adds up to 50 CRS points: 25 base, plus 25 more if your English is also CLB 5 or higher." },
      { type: "choice", audio: "n3u01-q04", q: "In the listening section, how many times do you hear each recording?", choices: ["Once", "Twice", "Three times", "As many times as you want"], answer: 0, explain: "Once only — which is why you read the questions BEFORE the audio starts." },
      { type: "choice", audio: "n3u01-q05", q: "Is there a penalty for wrong answers in the listening and reading sections?", choices: ["No — always answer every question", "Yes, minus one point per mistake", "Yes, but only in reading", "Only if you finish early"], answer: 0, explain: "No negative marking in TEF Canada: a guess can only help you. Never leave a blank." },
      { type: "choice", audio: "n3u01-q06", q: "How long are TEF Canada results valid for immigration?", choices: ["2 years", "1 year", "5 years", "Forever"], answer: 0, explain: "Two years from the test date — plan your Express Entry profile timing accordingly." }
    ]
  },
  {
    id: "n3u02", level: 3,
    title: "Listening: beat the traps",
    titleFr: "Compréhension orale : déjouer les pièges",
    icon: "🎧", color: "#4A90D9",
    objectives: [
      "Read the questions and answers BEFORE the audio starts",
      "Spot the classic traps: numbers, dates, and negatives",
      "Take quick notes on key facts while listening"
    ],
    imagePrompt: "flat vector illustration of oversized headphones above a speech bubble with sound waves and a question mark, listening test concept, clean minimal style, blue accents",
    vocab: [
      { fr: "une quinzaine", en: "about fifteen", tip: "Not exactly 15 — approximately!", audio: "n3u02-v01" },
      { fr: "la veille", en: "the day before", audio: "n3u02-v02" },
      { fr: "le lendemain", en: "the next day", audio: "n3u02-v03" },
      { fr: "avoir lieu", en: "to take place", audio: "n3u02-v04" },
      { fr: "reporter", en: "to postpone", tip: "reporTER = push it laTER", audio: "n3u02-v05" },
      { fr: "faute de", en: "for lack of / due to a lack of", audio: "n3u02-v06" },
      { fr: "désormais", en: "from now on", audio: "n3u02-v07" },
      { fr: "un créneau", en: "a time slot", audio: "n3u02-v08" },
      { fr: "à moitié prix", en: "half price", audio: "n3u02-v09" },
      { fr: "quatre-vingt-dix", en: "ninety", tip: "Literally 'four-twenties-ten' — a classic number trap", audio: "n3u02-v10" }
    ],
    phrases: [
      { fr: "Ce qui est dit :", en: "What is said:", audio: "n3u02-p01" },
      { fr: "Ce qui est demandé :", en: "What is asked:", audio: "n3u02-p02" },
      { fr: "Le piège, c'est...", en: "The trap is...", audio: "n3u02-p03" },
      { fr: "La bonne réponse est...", en: "The right answer is...", audio: "n3u02-p04" },
      { fr: "On entend d'abord..., mais ensuite...", en: "We first hear..., but then...", audio: "n3u02-p05" },
      { fr: "Attention à la négation !", en: "Watch out for the negative!", audio: "n3u02-p06" }
    ],
    pronunciation: {
      title: "Listening strategy: before, during, after",
      points: [
        "BEFORE the audio: read the question and all four answers. Underline key words — numbers, dates, names.",
        "DURING: take short notes, especially numbers, days, and anything negative (pas, jamais, ne... plus).",
        "AFTER: eliminate answers that contradict what you heard. If unsure, guess — there is no penalty."
      ]
    },
    grammar: {
      title: "The distractor trap",
      explanation: "Wrong answers in TEF listening are not random — they are built to trap you. One classic trap repeats a word from the recording but changes its meaning. Another gives the right information with the wrong number or date. A third states the opposite of what was said, hoping you missed a negative like 'ne... plus'. Your defense: match the MEANING, not the words.",
      examples: [
        { fr: "Le concert est reporté au samedi suivant.", en: "The concert is postponed to the following Saturday.", audio: "n3u02-g01" },
        { fr: "L'entrée est gratuite pour les moins de douze ans.", en: "Entry is free for children under twelve.", audio: "n3u02-g02" },
        { fr: "Le train ne s'arrête plus à cette gare.", en: "The train no longer stops at this station.", audio: "n3u02-g03" }
      ]
    },
    dialogue: {
      title: "Sample: a phone message",
      lines: [
        { speaker: "A", fr: "Bonjour, ici la clinique Saint-Laurent.", en: "Hello, this is the Saint-Laurent clinic.", audio: "n3u02-d01" },
        { speaker: "A", fr: "Nous devons reporter votre rendez-vous du 12 mars.", en: "We have to postpone your appointment of March 12.", audio: "n3u02-d02" },
        { speaker: "A", fr: "Le docteur sera absent toute la semaine.", en: "The doctor will be away all week.", audio: "n3u02-d03" },
        { speaker: "A", fr: "Nous vous proposons le 21 mars à quatorze heures trente.", en: "We can offer you March 21 at 2:30 p.m.", audio: "n3u02-d04" },
        { speaker: "A", fr: "Merci de confirmer au 514-555-0182.", en: "Please confirm at 514-555-0182.", audio: "n3u02-d05" }
      ]
    },
    quiz: [
      { type: "listening", audio: "n3u02-q01", q: "From which platform does the 8:47 train to Québec leave?", transcript: "Attention, voyageurs : le train de 8 h 47 à destination de Québec partira du quai numéro 5, et non du quai numéro 3 comme annoncé précédemment.", choices: ["Platform 5", "Platform 3", "Platform 8", "The announcement doesn't say"], answer: 0, explain: "Trap: 'quai numéro 3' is mentioned — but only to correct it. The train leaves from platform 5." },
      { type: "listening", audio: "n3u02-q02", q: "When is the museum open on July 14?", transcript: "Le musée sera fermé le lundi, comme d'habitude, mais il ouvrira exceptionnellement ce lundi 14 juillet de 10 h à 18 h.", choices: ["10 a.m. to 6 p.m.", "It is closed", "10 a.m. to 8 p.m.", "Only in the morning"], answer: 0, explain: "Trap: 'fermé le lundi' — but this Monday is an exception. Listen for 'exceptionnellement'." },
      { type: "listening", audio: "n3u02-q03", q: "Will the woman go to the party?", transcript: "— Tu viens à la fête samedi ? — J'aimerais bien, mais je travaille jusqu'à 22 h. — Pas de problème, on commence à 21 h mais ça finit tard !", choices: ["Yes, after work", "No, she works too late", "Yes, at 9 p.m.", "She hasn't decided"], answer: 0, explain: "She works until 10 p.m., but the party 'finit tard' (ends late) — so she can come after work." },
      { type: "listening", audio: "n3u02-q04", q: "Why are the Toronto flights cancelled?", transcript: "En raison d'une grève, les vols à destination de Toronto sont annulés aujourd'hui. Les passagers sont priés de contacter la compagnie.", choices: ["A strike", "Bad weather", "A technical problem", "The announcement doesn't say"], answer: 0, explain: "'En raison d'une grève' = due to a strike. Don't invent other reasons." },
      { type: "listening", audio: "n3u02-q05", q: "What is the monthly rent?", transcript: "— L'appartement fait combien de mètres carrés ? — Quatre-vingt-dix. — Et le loyer ? — Mille deux cents dollars par mois, charges comprises.", choices: ["$1,200 including charges", "$900", "$1,200 excluding charges", "$2,000"], answer: 0, explain: "Number trap: 90 (quatre-vingt-dix) is the size in m², not the price. 'Charges comprises' = charges included." },
      { type: "listening", audio: "n3u02-q06", q: "When is the duty pharmacy open on Sunday?", transcript: "La pharmacie de garde ce week-end est la pharmacie Centrale, rue Sainte-Catherine. Elle est ouverte samedi de 9 h à 20 h et dimanche de 10 h à 18 h.", choices: ["10 a.m. to 6 p.m.", "9 a.m. to 8 p.m.", "It is closed on Sunday", "9 a.m. to 6 p.m."], answer: 0, explain: "Trap: Saturday's hours (9 a.m.–8 p.m.) are stated first. Sunday is 10 a.m.–6 p.m." }
    ]
  },
  {
    id: "n3u03", level: 3,
    title: "Reading: speed and precision",
    titleFr: "Compréhension écrite : vite et juste",
    icon: "📖", color: "#2E9E5B",
    objectives: [
      "Skim for structure before reading in detail",
      "Answer chronology and inference questions",
      "Manage 60 minutes across 50 questions without getting stuck"
    ],
    imagePrompt: "flat vector illustration of an open newspaper with a magnifying glass over lines of text, reading test concept, clean minimal style, green accents",
    vocab: [
      { fr: "selon", en: "according to", audio: "n3u03-v01" },
      { fr: "toutefois", en: "however", audio: "n3u03-v02" },
      { fr: "il s'agit de", en: "it is about / it concerns", audio: "n3u03-v03" },
      { fr: "un sondage", en: "a survey / a poll", audio: "n3u03-v04" },
      { fr: "un enjeu", en: "an issue / what's at stake", tip: "Les enjeux = the key issues", audio: "n3u03-v05" },
      { fr: "souligner", en: "to emphasize / to underline", audio: "n3u03-v06" },
      { fr: "dresser un bilan", en: "to draw up an assessment", audio: "n3u03-v07" },
      { fr: "à l'instar de", en: "following the example of", audio: "n3u03-v08" },
      { fr: "sauf", en: "except", tip: "In questions, 'sauf' reverses the whole logic!", audio: "n3u03-v09" },
      { fr: "dorénavant", en: "from now on", audio: "n3u03-v10" }
    ],
    phrases: [
      { fr: "D'après le texte,", en: "According to the text,", audio: "n3u03-p01" },
      { fr: "On peut en déduire que...", en: "We can deduce that...", audio: "n3u03-p02" },
      { fr: "L'auteur suggère que...", en: "The author suggests that...", audio: "n3u03-p03" },
      { fr: "Le texte porte principalement sur...", en: "The text is mainly about...", audio: "n3u03-p04" },
      { fr: "En d'autres termes,", en: "In other words,", audio: "n3u03-p05" },
      { fr: "Contrairement à ce qui est dit,", en: "Contrary to what is said,", audio: "n3u03-p06" }
    ],
    pronunciation: {
      title: "Exam tips: reading",
      points: [
        "Read the QUESTIONS first, then skim the text for the answers — don't translate every word.",
        "Watch for 'sauf', 'à l'exception de' in questions: they reverse the logic of the question.",
        "For chronology questions, circle every date and time marker as you read."
      ]
    },
    grammar: {
      title: "Reading for inference — practice text 1",
      explanation: "Some questions ask what is directly stated; others ask what you can INFER — what the text suggests without saying it outright. Inference answers are never wild guesses: they combine two or three clues from the text. And beware of absolutes: if an answer choice says 'toujours' (always) or 'jamais' (never), be suspicious — texts rarely support them. Read the text below, then answer the quiz.",
      examples: [
        { fr: "À Montréal, le vélo-partage connaît un succès grandissant. Selon un sondage publié hier, plus de 60 % des utilisateurs prennent un vélo au moins trois fois par semaine.", en: "In Montréal, bike-sharing is enjoying growing success. According to a survey published yesterday, more than 60% of users take a bike at least three times a week.", audio: "n3u03-g01" },
        { fr: "Toutefois, le service fait face à un enjeu de taille : en hiver, le nombre d'utilisateurs chute de 80 %. La ville réfléchit donc à des vélos adaptés au froid.", en: "However, the service faces a major challenge: in winter, the number of users drops by 80%. The city is therefore considering bikes adapted to the cold.", audio: "n3u03-g02" },
        { fr: "« Nous voulons que le vélo reste une option toute l'année », souligne la mairesse. Un projet pilote sera lancé en janvier dans trois quartiers.", en: "'We want cycling to remain an option all year round,' the mayor emphasizes. A pilot project will be launched in January in three neighbourhoods.", audio: "n3u03-g03" },
        { fr: "À l'instar de plusieurs villes européennes, Montréal espère ainsi réduire la circulation automobile de 15 % d'ici 2030.", en: "Following the example of several European cities, Montréal hopes to reduce car traffic by 15% by 2030.", audio: "n3u03-g04" }
      ]
    },
    dialogue: {
      title: "Practice text 2: community announcement",
      lines: [
        { speaker: "A", fr: "La Maison des jeunes de Verdun propose de nouvelles activités pour l'automne.", en: "The Verdun youth centre is offering new activities for the fall.", audio: "n3u03-d01" },
        { speaker: "A", fr: "Au programme : un atelier de cuisine les mardis soirs, un club de lecture les jeudis, et des sorties en plein air un samedi par mois.", en: "On the program: a cooking workshop on Tuesday evenings, a reading club on Thursdays, and outdoor outings one Saturday a month.", audio: "n3u03-d02" },
        { speaker: "A", fr: "Les inscriptions sont ouvertes jusqu'au 30 septembre. Attention : les places sont limitées à vingt participants par activité.", en: "Registration is open until September 30. Note: places are limited to twenty participants per activity.", audio: "n3u03-d03" },
        { speaker: "A", fr: "Contrairement à l'an dernier, toutes les activités seront gratuites cette année grâce à une subvention de la ville.", en: "Unlike last year, all activities will be free this year thanks to a city grant.", audio: "n3u03-d04" }
      ]
    },
    quiz: [
      { type: "choice", audio: "n3u03-q01", q: "Text 1: how often do most bike-share users ride?", choices: ["At least 3 times a week", "Every day", "Once a month", "Only in winter"], answer: 0, explain: "'au moins trois fois par semaine' — at least three times a week." },
      { type: "choice", audio: "n3u03-q02", q: "Text 1: what is the main challenge ('enjeu') mentioned?", choices: ["The 80% drop in users in winter", "The price of the service", "A lack of bicycles", "Road accidents"], answer: 0, explain: "The text names exactly one enjeu: winter use falls by 80%." },
      { type: "choice", audio: "n3u03-q03", q: "Text 1: what can be deduced about the mayor's position?", choices: ["She supports cycling all year round", "She opposes the bike-share program", "She wants to close the service in winter", "She hasn't expressed an opinion"], answer: 0, explain: "'Nous voulons que le vélo reste une option toute l'année' — she is clearly in favour." },
      { type: "choice", audio: "n3u03-q04", q: "Text 2: how often are the outdoor outings?", choices: ["Once a month, on Saturdays", "Every Saturday", "Every Thursday", "Twice a month"], answer: 0, explain: "'un samedi par mois' — one Saturday per month." },
      { type: "choice", audio: "n3u03-q05", q: "Text 2: what is NEW this year at the youth centre?", choices: ["All activities are free", "There is a cooking workshop", "Registration closes September 30", "Places are limited to 20"], answer: 0, explain: "'Contrairement à l'an dernier, toutes les activités seront gratuites' — free access is the change." },
      { type: "choice", audio: "n3u03-q06", q: "Text 2: which activity takes place on Tuesdays?", choices: ["The cooking workshop", "The reading club", "The outdoor outings", "None — the centre is closed on Tuesdays"], answer: 0, explain: "'un atelier de cuisine les mardis soirs'." }
    ]
  },
  {
    id: "n3u04", level: 3,
    title: "Speaking: convince them",
    titleFr: "Expression orale : convaincre",
    icon: "🎙️", color: "#9B59B6",
    objectives: [
      "Section A: ask open, polite questions to get information (5 min)",
      "Section B: deliver a structured persuasive speech — opinion, arguments, example, conclusion (10 min)",
      "Use 10+ connectors naturally while speaking"
    ],
    imagePrompt: "flat vector illustration of a confident person speaking into a microphone with speech bubbles and checkmarks, speaking test concept, clean minimal style, purple accents",
    vocab: [
      { fr: "Pourriez-vous me dire...", en: "Could you tell me...", tip: "The conditionnel makes it polite", audio: "n3u04-v01" },
      { fr: "Est-ce que vous savez si...", en: "Do you know if...", audio: "n3u04-v02" },
      { fr: "À quelle fréquence... ?", en: "How often...?", audio: "n3u04-v03" },
      { fr: "Combien de temps... ?", en: "How long...?", audio: "n3u04-v04" },
      { fr: "Quelles sont les conditions pour... ?", en: "What are the requirements for...?", audio: "n3u04-v05" },
      { fr: "Y a-t-il... ?", en: "Is there / Are there...?", tip: "Inversion with 'il' — formal and safe", audio: "n3u04-v06" },
      { fr: "un argument", en: "an argument", audio: "n3u04-v07" },
      { fr: "un inconvénient", en: "a drawback", audio: "n3u04-v08" },
      { fr: "un avantage", en: "an advantage", audio: "n3u04-v09" },
      { fr: "prendre position", en: "to take a stand", audio: "n3u04-v10" }
    ],
    phrases: [
      { fr: "À mon avis,", en: "In my opinion,", audio: "n3u04-p01" },
      { fr: "Selon moi,", en: "In my view,", audio: "n3u04-p02" },
      { fr: "D'une part..., d'autre part...", en: "On the one hand..., on the other hand...", audio: "n3u04-p03" },
      { fr: "Prenons l'exemple de...", en: "Take the example of...", audio: "n3u04-p04" },
      { fr: "C'est pourquoi je pense que...", en: "That's why I think that...", audio: "n3u04-p05" },
      { fr: "Pour conclure,", en: "To conclude,", audio: "n3u04-p06" },
      { fr: "Je suis convaincu que...", en: "I'm convinced that...", audio: "n3u04-p07" },
      { fr: "Il ne faut pas oublier que...", en: "We mustn't forget that...", audio: "n3u04-p08" }
    ],
    pronunciation: {
      title: "Speaking tips: exam day",
      points: [
        "Section A (5 min): ask as many questions as you can. Open questions (pourquoi, comment) earn more than yes/no ones.",
        "Section B (10 min): keep talking for the full time. Silence is the only real failure.",
        "Slow down and articulate. A calm, clear speaker scores higher than a fast, mumbling one."
      ]
    },
    grammar: {
      title: "The persuasion framework (Section B)",
      explanation: "In Section B you must defend an opinion for several minutes. Use this framework every single time: 1) state your opinion clearly, 2) give TWO arguments, each introduced by a connector, 3) illustrate with a concrete example, 4) conclude by restating your opinion. Examiners reward structure as much as vocabulary — a well-organized B1 speech beats a chaotic B2 one.",
      examples: [
        { fr: "À mon avis, le télétravail est une excellente chose.", en: "In my opinion, teleworking is an excellent thing.", audio: "n3u04-g01" },
        { fr: "D'une part, il fait gagner du temps ; d'autre part, il réduit le stress.", en: "On the one hand, it saves time; on the other hand, it reduces stress.", audio: "n3u04-g02" },
        { fr: "Prenons l'exemple de mon voisin : il a retrouvé deux heures par jour.", en: "Take my neighbour's example: he got back two hours a day.", audio: "n3u04-g03" },
        { fr: "C'est pourquoi je suis convaincu que le télétravail est l'avenir.", en: "That's why I'm convinced teleworking is the future.", audio: "n3u04-g04" }
      ]
    },
    dialogue: {
      title: "3 model mini-speeches — repeat after them",
      lines: [
        { speaker: "A", fr: "À mon avis, le télétravail devrait être proposé à tous les employés de bureau.", en: "In my opinion, teleworking should be offered to all office employees.", audio: "n3u04-d01" },
        { speaker: "A", fr: "D'une part, on évite les transports, qui sont fatigants et polluants. D'autre part, on est souvent plus concentré chez soi.", en: "On the one hand, you avoid commuting, which is tiring and polluting. On the other hand, you're often more focused at home.", audio: "n3u04-d02" },
        { speaker: "A", fr: "Prenons l'exemple de mon amie Julie : depuis qu'elle travaille à la maison, elle est moins stressée et plus efficace. Pour conclure, le télétravail est bon pour les employés et pour la planète.", en: "Take my friend Julie's example: since she started working from home, she's less stressed and more efficient. To conclude, teleworking is good for employees and for the planet.", audio: "n3u04-d03" },
        { speaker: "A", fr: "Selon moi, il faudrait interdire les voitures au centre-ville.", en: "In my view, cars should be banned from downtown.", audio: "n3u04-d04" },
        { speaker: "A", fr: "Tout d'abord, la pollution de l'air est dangereuse pour la santé. De plus, les rues seraient plus sûres pour les piétons et les cyclistes.", en: "First of all, air pollution is dangerous for health. Furthermore, streets would be safer for pedestrians and cyclists.", audio: "n3u04-d05" },
        { speaker: "A", fr: "À Montréal, certaines rues sont déjà piétonnes l'été, et les commerçants sont satisfaits. C'est pourquoi je pense que cette solution est l'avenir de nos villes.", en: "In Montréal, some streets are already pedestrian-only in summer, and shopkeepers are satisfied. That's why I think this solution is the future of our cities.", audio: "n3u04-d06" },
        { speaker: "A", fr: "Il me semble évident que les enfants devraient lire plus.", en: "It seems obvious to me that children should read more.", audio: "n3u04-d07" },
        { speaker: "A", fr: "D'une part, la lecture développe le vocabulaire et l'imagination. D'autre part, elle éloigne les enfants des écrans, au moins un moment.", en: "On the one hand, reading develops vocabulary and imagination. On the other hand, it keeps children away from screens, at least for a while.", audio: "n3u04-d08" },
        { speaker: "A", fr: "Par exemple, mon neveu lit vingt minutes chaque soir, et ses notes se sont améliorées. En conclusion, un livre par mois peut changer la vie d'un enfant.", en: "For example, my nephew reads twenty minutes every evening, and his grades have improved. In conclusion, one book a month can change a child's life.", audio: "n3u04-d09" }
      ]
    },
    quiz: [
      { type: "choice", audio: "n3u04-q01", q: "Section A: you must get information about a language school. Which question is BEST?", choices: ["Pourriez-vous me dire quels cours vous proposez et à quels horaires ?", "Vous avez des cours ?", "C'est combien ?", "L'école est bien ?"], answer: 0, explain: "Open + polite (conditionnel) beats closed yes/no questions — it forces a developed answer." },
      { type: "choice", audio: "n3u04-q02", q: "Which of these is a CLOSED question (avoid it in Section A)?", choices: ["Est-ce que l'école est ouverte le samedi ?", "Quelles activités proposez-vous le soir ?", "Comment se déroule l'inscription ?", "Pourquoi les prix ont-ils augmenté ?"], answer: 0, explain: "Yes/no questions close the conversation. Prefer 'comment', 'pourquoi', 'quels'." },
      { type: "choice", audio: "n3u04-q03", q: "For Section B, which opening is strongest?", choices: ["À mon avis, les transports en commun devraient être gratuits.", "Les transports... euh... c'est... bien.", "Je ne sais pas quoi dire sur ce sujet.", "Les transports en commun existent dans les villes."], answer: 0, explain: "State a clear opinion immediately — the examiner needs a position to evaluate." },
      { type: "choice", audio: "n3u04-q04", q: "Which sentence makes the best ARGUMENT?", choices: ["D'une part, c'est moins cher ; d'autre part, c'est moins polluant.", "J'aime les transports.", "Les transports sont des transports.", "Mon ami prend le bus."], answer: 0, explain: "A real argument gives reasons, structured with connectors." },
      { type: "choice", audio: "n3u04-q05", q: "Which is the best concrete EXAMPLE?", choices: ["Prenons l'exemple de Lyon : depuis la gratuité, le nombre de voyageurs a augmenté de 30 %.", "Par exemple, c'est bien.", "Exemple : les bus.", "J'ai un exemple mais je l'ai oublié."], answer: 0, explain: "Concrete and precise (a place, a number) = convincing." },
      { type: "choice", audio: "n3u04-q06", q: "Which conclusion works best?", choices: ["Pour conclure, je reste convaincu que la gratuité est une solution d'avenir.", "Voilà, c'est fini.", "Je n'ai plus rien à dire.", "Conclusion : je ne sais pas."], answer: 0, explain: "Restate your opinion with a connector — never end with 'I have nothing more to say'." }
    ]
  },
  {
    id: "n3u05", level: 3,
    title: "Writing: letters that score",
    titleFr: "Expression écrite : des lettres qui rapportent",
    icon: "✍️", color: "#E67E22",
    objectives: [
      "Section A: write a fait divers (80–100 words) answering who, what, where, when",
      "Section B: write a formal letter (200 words) with correct formules de politesse",
      "Use 15 key connectors accurately and naturally"
    ],
    imagePrompt: "flat vector illustration of a hand writing a formal letter with a fountain pen, an envelope and a stamp nearby, writing test concept, clean minimal style, orange accents",
    vocab: [
      { fr: "un fait divers", en: "a news brief", audio: "n3u05-v01" },
      { fr: "un témoin", en: "a witness", audio: "n3u05-v02" },
      { fr: "les secours (m. pl.)", en: "the emergency services", audio: "n3u05-v03" },
      { fr: "porter plainte", en: "to file a complaint", audio: "n3u05-v04" },
      { fr: "l'objet (m.) d'une lettre", en: "the subject line of a letter", tip: "Objet : demande d'information", audio: "n3u05-v05" },
      { fr: "ci-joint", en: "attached / enclosed", audio: "n3u05-v06" },
      { fr: "dans l'attente de votre réponse", en: "awaiting your reply", audio: "n3u05-v07" },
      { fr: "un destinataire", en: "a recipient / addressee", audio: "n3u05-v08" },
      { fr: "une réclamation", en: "a (formal) complaint", audio: "n3u05-v09" },
      { fr: "cordialement", en: "best regards", tip: "Friendly — for colleagues, NOT for formal complaints", audio: "n3u05-v10" }
    ],
    phrases: [
      { fr: "Tout d'abord,", en: "First of all,", audio: "n3u05-p01" },
      { fr: "De plus,", en: "Furthermore,", audio: "n3u05-p02" },
      { fr: "En outre,", en: "Moreover,", audio: "n3u05-p03" },
      { fr: "Cependant,", en: "However,", audio: "n3u05-p04" },
      { fr: "Toutefois,", en: "Nevertheless,", audio: "n3u05-p05" },
      { fr: "Néanmoins,", en: "Nonetheless,", audio: "n3u05-p06" },
      { fr: "Par conséquent,", en: "Consequently,", audio: "n3u05-p07" },
      { fr: "C'est pourquoi...", en: "That's why...", audio: "n3u05-p08" },
      { fr: "En revanche,", en: "On the other hand,", audio: "n3u05-p09" },
      { fr: "D'une part... d'autre part...", en: "On the one hand... on the other hand...", audio: "n3u05-p10" },
      { fr: "En effet,", en: "Indeed,", audio: "n3u05-p11" },
      { fr: "Par exemple,", en: "For example,", audio: "n3u05-p12" },
      { fr: "Il est important de noter que...", en: "It is important to note that...", audio: "n3u05-p13" },
      { fr: "En conclusion,", en: "In conclusion,", audio: "n3u05-p14" },
      { fr: "Je vous prie d'agréer, Madame, Monsieur, l'expression de mes salutations distinguées.", en: "Please accept, Madam, Sir, the expression of my distinguished regards.", tip: "The classic formal closing — memorize it word for word", audio: "n3u05-p15" }
    ],
    pronunciation: {
      title: "Exam tips: writing",
      points: [
        "Section A ≈ 80 words minimum (aim for ~100); Section B ≈ 200 words minimum. Count roughly as you write.",
        "Keep 5 minutes at the end to re-read: check verb endings, agreements, and accents.",
        "Never use 'tu' in Section B — a formal letter is always 'vous'."
      ]
    },
    grammar: {
      title: "Section A: the fait divers method",
      explanation: "Section A asks for a short newspaper article (un fait divers) of about 80 words minimum. Method: answer four questions — QUI (who), QUOI (what), OÙ (where), QUAND (when) — then add one detail (a witness, a consequence, an investigation). Write in the passé composé, keep sentences short, and give your article a title. Below is a full model: read it, shadow the audio, then write your own on a different event.",
      examples: [
        { fr: "INCENDIE DANS UN IMMEUBLE DU PLATEAU", en: "FIRE IN A PLATEAU APARTMENT BUILDING", audio: "n3u05-g01" },
        { fr: "Hier soir, vers 21 h, un incendie s'est déclaré dans un immeuble de trois étages de la rue Rachel, à Montréal.", en: "Last night, around 9 p.m., a fire broke out in a three-storey building on Rachel Street in Montréal.", audio: "n3u05-g02" },
        { fr: "Selon les témoins, les flammes sont parties d'une cuisine au deuxième étage. Les secours, arrivés rapidement, ont évacué douze personnes. Aucun blessé grave n'est à déplorer.", en: "According to witnesses, the flames started in a second-floor kitchen. Emergency services, arriving quickly, evacuated twelve people. No serious injuries are reported.", audio: "n3u05-g03" },
        { fr: "L'origine du sinistre reste inconnue. Une enquête est en cours.", en: "The cause of the fire remains unknown. An investigation is underway.", audio: "n3u05-g04" }
      ]
    },
    dialogue: {
      title: "Section B: model formal letter",
      lines: [
        { speaker: "A", fr: "Objet : nuisances sonores — demande d'intervention", en: "Subject: noise disturbance — request for action", audio: "n3u05-d01" },
        { speaker: "A", fr: "Madame, Monsieur,", en: "Dear Madam, Dear Sir,", audio: "n3u05-d02" },
        { speaker: "A", fr: "Je me permets de vous écrire au sujet des travaux qui ont lieu dans l'immeuble depuis le 2 septembre.", en: "I am writing to you regarding the construction work taking place in the building since September 2.", audio: "n3u05-d03" },
        { speaker: "A", fr: "En effet, le bruit commence chaque jour à 6 h 30 du matin, ce qui m'empêche de dormir et de travailler. De plus, la poussière rend l'escalier difficile d'accès pour les personnes âgées.", en: "Indeed, the noise starts every day at 6:30 a.m., which prevents me from sleeping and working. Furthermore, the dust makes the stairwell difficult to access for elderly people.", audio: "n3u05-d04" },
        { speaker: "A", fr: "C'est pourquoi je vous demande de bien vouloir faire respecter les horaires légaux, soit de 8 h à 18 h en semaine.", en: "That is why I ask you to please enforce the legal working hours, namely 8 a.m. to 6 p.m. on weekdays.", audio: "n3u05-d05" },
        { speaker: "A", fr: "Dans l'attente de votre réponse, je vous prie d'agréer, Madame, Monsieur, l'expression de mes salutations distinguées.", en: "Awaiting your reply, please accept, Madam, Sir, the expression of my distinguished regards.", audio: "n3u05-d06" },
        { speaker: "A", fr: "Marie Tremblay, appartement 12", en: "Marie Tremblay, apartment 12", audio: "n3u05-d07" }
      ]
    },
    quiz: [
      { type: "choice", audio: "n3u05-q01", q: "Complete the formal letter closing: « Je vous prie d'agréer, Madame, Monsieur, ... »", choices: ["l'expression de mes salutations distinguées.", "mes amitiés.", "à bientôt !", "cordialement, bisous."], answer: 0, explain: "The full formal formula is expected in Section B — memorize it exactly." },
      { type: "choice", audio: "n3u05-q02", q: "Which connector introduces a CONSEQUENCE?", choices: ["Par conséquent,", "Cependant,", "Par exemple,", "Tout d'abord,"], answer: 0, explain: "'Par conséquent' = therefore. 'Cependant' = however (opposition); 'par exemple' = example." },
      { type: "choice", audio: "n3u05-q03", q: "In a formal complaint letter, which greeting is correct?", choices: ["Madame, Monsieur,", "Salut !", "Cher ami,", "Coucou,"], answer: 0, explain: "Formal letters open with 'Madame, Monsieur,' — never casual greetings or 'tu' forms." },
      { type: "choice", audio: "n3u05-q04", q: "Choose the sentence with the correct formal 'vous' form:", choices: ["Je vous demande de bien vouloir intervenir.", "Je te demande d'intervenir vite.", "Tu dois faire quelque chose.", "Interviens s'il te plaît."], answer: 0, explain: "Section B is always 'vous'. 'Je vous demande de bien vouloir...' is the polite formal standard." },
      { type: "choice", audio: "n3u05-q05", q: "Which connector shows OPPOSITION?", choices: ["En revanche,", "De plus,", "En effet,", "C'est pourquoi"], answer: 0, explain: "'En revanche' = on the other hand. The others add information or give reasons." },
      { type: "choice", audio: "n3u05-q06", q: "Your fait divers is only 45 words. What should you do?", choices: ["Add a detail (a witness, a consequence) to reach about 80–100 words", "Submit it — short is fine", "Repeat the same sentence twice", "Write the title in bigger letters"], answer: 0, explain: "Section A needs about 80 words minimum — add one concrete detail: a witness, a consequence, or the investigation." }
    ]
  }
];
