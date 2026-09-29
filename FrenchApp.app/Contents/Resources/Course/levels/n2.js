const N2_UNITS = [
  {
    id: "n2u01", level: 2,
    title: "Passé composé (1)", titleFr: "Le passé composé avec avoir",
    icon: "⏪", color: "#7B61FF",
    objectives: [
      "Form the passé composé with avoir + past participle (j'ai mangé).",
      "Use regular participles (-é, -i, -u) and key irregulars: fait, pris, vu, dit, écrit.",
      "Talk about yesterday and last weekend in the past tense."
    ],
    imagePrompt: "flat vector illustration of a calendar page marked yesterday with a checkmark, a person thinking back over the weekend",
    vocab: [
      { fr: "hier", en: "yesterday", tip: "Pronounced like 'ee-air' — the h is silent.", audio: "n2u01-v01" },
      { fr: "le week-end", en: "the weekend", audio: "n2u01-v02" },
      { fr: "déjà", en: "already", tip: "Goes after the verb: j'ai déjà mangé.", audio: "n2u01-v03" },
      { fr: "un film", en: "a movie, a film", audio: "n2u01-v04" },
      { fr: "inviter", en: "to invite", audio: "n2u01-v05" },
      { fr: "un ami / une amie", en: "a friend (male / female)", audio: "n2u01-v06" },
      { fr: "ensemble", en: "together", audio: "n2u01-v07" },
      { fr: "tard", en: "late", audio: "n2u01-v08" },
      { fr: "tôt", en: "early", tip: "Opposite of tard. The ô has a little hat: circumflex.", audio: "n2u01-v09" },
      { fr: "un message", en: "a message, a text", audio: "n2u01-v10" },
      { fr: "finir", en: "to finish", tip: "Participle: fini.", audio: "n2u01-v11" },
      { fr: "passer", en: "to spend (time)", tip: "Passer un bon week-end = to have a good weekend.", audio: "n2u01-v12" }
    ],
    phrases: [
      { fr: "Qu'est-ce que tu as fait hier ?", en: "What did you do yesterday?", audio: "n2u01-p01" },
      { fr: "J'ai regardé un film.", en: "I watched a movie.", audio: "n2u01-p02" },
      { fr: "Nous avons mangé ensemble.", en: "We ate together.", audio: "n2u01-p03" },
      { fr: "Elle a fini tard.", en: "She finished late.", audio: "n2u01-p04" },
      { fr: "Ils ont pris le bus.", en: "They took the bus.", audio: "n2u01-p05" },
      { fr: "J'ai vu un bon film.", en: "I saw a good movie.", audio: "n2u01-p06" },
      { fr: "Tu as dit quoi ?", en: "What did you say? (informal)", audio: "n2u01-p07" },
      { fr: "On a bien mangé !", en: "We had a great meal!", audio: "n2u01-p08" }
    ],
    pronunciation: {
      title: "The silent endings of the past",
      points: [
        "Past participles in -é sound like 'ay': regardé = ruh-gar-DAY. Never pronounce the final consonant that isn't there.",
        "In 'ils ont fini', the t of 'ont' is silent: eel zon fee-NEE. Link it smoothly, don't pop the t.",
        "J'ai is one single sound: 'zhay'. Don't say jeh-ah-ee — melt it together."
      ]
    },
    grammar: {
      title: "Passé composé with avoir: j'ai + past participle",
      explanation: "To talk about a finished action in the past, use the present of avoir plus the past participle: j'ai mangé (I ate / I have eaten). Regular verbs: -er becomes -é (parler → parlé), -ir becomes -i (finir → fini), -re becomes -u (vendre → vendu). Memorize the irregulars — the exam loves them: faire → fait, prendre → pris, voir → vu, dire → dit, écrire → écrit.",
      examples: [
        { fr: "J'ai parlé avec mon ami.", en: "I spoke with my friend.", audio: "n2u01-g01" },
        { fr: "Elle a fini à six heures.", en: "She finished at six o'clock.", audio: "n2u01-g02" },
        { fr: "Nous avons pris le bus.", en: "We took the bus.", audio: "n2u01-g03" },
        { fr: "Ils ont vu un film hier.", en: "They saw a movie yesterday.", audio: "n2u01-g04" }
      ]
    },
    dialogue: {
      title: "Le week-end de Marie",
      lines: [
        { speaker: "A", fr: "Salut, Marie ! Tu as passé un bon week-end ?", en: "Hi Marie! Did you have a good weekend?", audio: "n2u01-d01" },
        { speaker: "B", fr: "Oui, très bon ! Et toi ?", en: "Yes, very good! And you?", audio: "n2u01-d02" },
        { speaker: "A", fr: "Très bon aussi. Qu'est-ce que tu as fait samedi ?", en: "Very good too. What did you do on Saturday?", audio: "n2u01-d03" },
        { speaker: "B", fr: "J'ai invité mes amis et nous avons regardé un film ensemble.", en: "I invited my friends and we watched a movie together.", audio: "n2u01-d04" },
        { speaker: "A", fr: "C'était un bon film ?", en: "Was it a good movie?", audio: "n2u01-d05" },
        { speaker: "B", fr: "Oui, excellent ! Nous avons fini tard.", en: "Yes, excellent! We finished late.", audio: "n2u01-d06" },
        { speaker: "A", fr: "Et dimanche, tu as fait quoi ?", en: "And on Sunday, what did you do?", audio: "n2u01-d07" },
        { speaker: "B", fr: "Dimanche, j'ai écrit un message à ma mère et nous avons mangé ensemble.", en: "On Sunday, I wrote a message to my mother and we ate together.", audio: "n2u01-d08" },
        { speaker: "A", fr: "Moi, j'ai invité des amis aussi. On a bien mangé !", en: "I invited friends too. We had a great meal!", audio: "n2u01-d09" },
        { speaker: "B", fr: "C'était un très bon week-end !", en: "It was a very good weekend!", audio: "n2u01-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose the correct passé composé of 'parler' with 'je'.", choices: ["j'ai parlé", "je suis parlé", "j'ai parler", "je parlé"], answer: 0, explain: "Passé composé = avoir + past participle. Parler is regular -er, so the participle is parlé: j'ai parlé." },
      { type: "choice", q: "Which past participle is correct for 'prendre'?", choices: ["pris", "prendu", "prené", "prit"], answer: 0, explain: "Prendre is irregular: prendre → pris. Memorize it — the exam tests it constantly." },
      { type: "choice", q: "Complete: Elle ___ un film hier.", choices: ["a regardé", "a regarder", "est regardée", "as regardé"], answer: 0, explain: "Regarder uses avoir, and the participle of an -er verb ends in -é: elle a regardé." },
      { type: "choice", q: "Which sentence is correct?", choices: ["Nous avons fini tôt.", "Nous avons finir tôt.", "Nous sommes fini tôt.", "Nous avons finis tôt."], answer: 0, explain: "With avoir, the participle never agrees with the subject — no extra -s. Nous avons fini tôt." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u01-q05", transcript: "J'ai pris le bus hier.", choices: ["I took the bus yesterday.", "I take the bus every day.", "I will take the bus tomorrow.", "I am taking the bus now."], answer: 0, explain: "J'ai pris = passé composé (past). Hier confirms it: I took the bus yesterday." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u01-q06", transcript: "Elle a écrit un message.", choices: ["She wrote a message.", "She is writing a message.", "She writes messages every day.", "She will write a message."], answer: 0, explain: "Elle a écrit = passé composé of écrire (irregular participle écrit): She wrote a message." }
    ]
  },
  {
    id: "n2u02", level: 2,
    title: "Passé composé (2)", titleFr: "Le passé composé avec être",
    icon: "🚶", color: "#00B8A9",
    objectives: [
      "Know the 17 être verbs (DR MRS VANDERTRAMP) and when to use être instead of avoir.",
      "Form the passé composé with être: elle est allée, ils sont partis.",
      "Make the past participle agree: -e for feminine, -s for plural."
    ],
    imagePrompt: "flat vector illustration of a person walking through a door, one arrow for arriving and one for leaving, movement verbs theme",
    vocab: [
      { fr: "aller", en: "to go", tip: "Participle: allé. Être verb!", audio: "n2u02-v01" },
      { fr: "venir", en: "to come", tip: "Irregular participle: venu.", audio: "n2u02-v02" },
      { fr: "arriver", en: "to arrive", audio: "n2u02-v03" },
      { fr: "partir", en: "to leave, to depart", audio: "n2u02-v04" },
      { fr: "entrer", en: "to enter, to go in", audio: "n2u02-v05" },
      { fr: "sortir", en: "to go out, to exit", audio: "n2u02-v06" },
      { fr: "monter", en: "to go up, to climb", audio: "n2u02-v07" },
      { fr: "descendre", en: "to go down", audio: "n2u02-v08" },
      { fr: "rester", en: "to stay, to remain", audio: "n2u02-v09" },
      { fr: "tomber", en: "to fall", audio: "n2u02-v10" },
      { fr: "rentrer", en: "to come back, to return home", audio: "n2u02-v11" },
      { fr: "naître", en: "to be born", tip: "Irregular participle: né. The î wears a circumflex hat.", audio: "n2u02-v12" }
    ],
    phrases: [
      { fr: "Elle est allée à Paris.", en: "She went to Paris.", audio: "n2u02-p01" },
      { fr: "Ils sont arrivés tard.", en: "They arrived late.", audio: "n2u02-p02" },
      { fr: "Nous sommes sortis ensemble.", en: "We went out together.", audio: "n2u02-p03" },
      { fr: "Je suis resté à la maison.", en: "I stayed home. (said by a man)", audio: "n2u02-p04" },
      { fr: "Tu es venu avec ton ami ?", en: "Did you come with your friend?", audio: "n2u02-p05" },
      { fr: "Elle est née en 1990.", en: "She was born in 1990.", audio: "n2u02-p06" },
      { fr: "Ils sont partis tôt.", en: "They left early.", audio: "n2u02-p07" },
      { fr: "Nous sommes rentrés tard.", en: "We came back late.", audio: "n2u02-p08" }
    ],
    pronunciation: {
      title: "Agreement you can hear — sometimes",
      points: [
        "The extra -e and -s of agreement are almost always SILENT: allé, allée, allés, allées all sound identical. Spelling carries the meaning.",
        "What you CAN hear is the auxiliary: 'il est allé' vs 'ils sont allés' — est (eh) vs sont (son). Train your ear on that difference.",
        "Liaison: in 'nous sommes‿arrivés', the s of 'sommes' links to the vowel with a z sound: noo som-zah-ree-VAY."
      ]
    },
    grammar: {
      title: "Être verbs: DR MRS VANDERTRAMP + agreement",
      explanation: "Seventeen verbs of movement or change of state use être, not avoir: Devenir, Revenir, Mourir, Retourner, Sortir, Venir, Aller, Naître, Descendre, Entrer, Rester, Tomber, Rentrer, Arriver, Monter, Partir — memorize them as DR MRS VANDERTRAMP. With être, the past participle AGREES with the subject: add -e for feminine, -s for plural. Il est allé, elle est allée, ils sont allés, elles sont allées.",
      examples: [
        { fr: "Il est allé en ville.", en: "He went downtown.", audio: "n2u02-g01" },
        { fr: "Elle est arrivée à huit heures.", en: "She arrived at eight o'clock.", audio: "n2u02-g02" },
        { fr: "Ils sont partis ensemble.", en: "They left together.", audio: "n2u02-g03" },
        { fr: "Nous sommes restés à la maison.", en: "We stayed home.", audio: "n2u02-g04" }
      ]
    },
    dialogue: {
      title: "Un week-end à Québec",
      lines: [
        { speaker: "A", fr: "Allô ? C'est Marc.", en: "Hello? It's Marc.", audio: "n2u02-d01" },
        { speaker: "B", fr: "Salut Marc ! Tu es allé où ce week-end ?", en: "Hi Marc! Where did you go this weekend?", audio: "n2u02-d02" },
        { speaker: "A", fr: "Je suis allé à Québec avec ma famille.", en: "I went to Quebec City with my family.", audio: "n2u02-d03" },
        { speaker: "B", fr: "Super ! Vous êtes partis quand ?", en: "Great! When did you leave?", audio: "n2u02-d04" },
        { speaker: "A", fr: "Nous sommes partis samedi matin et nous sommes rentrés dimanche soir.", en: "We left Saturday morning and came back Sunday evening.", audio: "n2u02-d05" },
        { speaker: "B", fr: "Et ta sœur ? Elle est venue avec vous ?", en: "And your sister? Did she come with you?", audio: "n2u02-d06" },
        { speaker: "A", fr: "Oui, elle est venue aussi. Elle est restée deux jours.", en: "Yes, she came too. She stayed two days.", audio: "n2u02-d07" },
        { speaker: "B", fr: "Vous êtes allés au restaurant ?", en: "Did you go to the restaurant?", audio: "n2u02-d08" },
        { speaker: "A", fr: "Oui ! Nous avons mangé ensemble. C'était très bon.", en: "Yes! We ate together. It was very good.", audio: "n2u02-d09" },
        { speaker: "B", fr: "C'est super !", en: "That's great!", audio: "n2u02-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Complete: Elle ___ à huit heures.", choices: ["est arrivée", "a arrivé", "est arrivé", "a arrivée"], answer: 0, explain: "Arriver is an être verb, and the subject is feminine singular: elle est arrivée (-e for feminine)." },
      { type: "choice", q: "Which verb uses ÊTRE in the passé composé?", choices: ["aller", "manger", "regarder", "finir"], answer: 0, explain: "Aller is a movement verb (DR MRS VANDERTRAMP). Manger, regarder, finir all use avoir." },
      { type: "choice", q: "Complete: Ils ___ partis tôt.", choices: ["sont", "ont", "est", "a"], answer: 0, explain: "Partir uses être: ils sont partis. Don't let the plural fool you into ont." },
      { type: "choice", q: "Choose the correct sentence.", choices: ["Elles sont allées en ville.", "Elles sont allé en ville.", "Elles ont allées en ville.", "Elles sont allés en ville."], answer: 0, explain: "Feminine plural subject → -es: elles sont allées. Aller takes être, never avoir." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u02-q05", transcript: "Elle est arrivée tard.", choices: ["She arrived late.", "He arrived late.", "They arrived late.", "She arrives late every day."], answer: 0, explain: "Elle est arrivée — feminine singular, passé composé with être: She arrived late." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u02-q06", transcript: "Nous sommes sortis ensemble.", choices: ["We went out together.", "We go out together every week.", "They went out together.", "We stayed home together."], answer: 0, explain: "Nous sommes sortis — passé composé of sortir with être: We went out together." }
    ]
  },
  {
    id: "n2u03", level: 2,
    title: "L'imparfait", titleFr: "L'imparfait et le passé composé",
    icon: "🌧️", color: "#5B8DEF",
    objectives: [
      "Form the imparfait from the 'nous' stem + -ais, -ait, -ions, -iez, -aient.",
      "Use the imparfait for habits, descriptions, and background situations.",
      "Choose correctly between passé composé and imparfait — the classic exam trap."
    ],
    imagePrompt: "flat vector illustration of a rainy window with a person remembering childhood, half memory and half present",
    vocab: [
      { fr: "pendant", en: "during", audio: "n2u03-v01" },
      { fr: "souvent", en: "often", audio: "n2u03-v02" },
      { fr: "chaque", en: "each, every", tip: "Chaque jour = every day.", audio: "n2u03-v03" },
      { fr: "autrefois", en: "in the past, formerly", audio: "n2u03-v04" },
      { fr: "d'habitude", en: "usually", audio: "n2u03-v05" },
      { fr: "le temps", en: "the weather; time", tip: "Quel temps fait-il ? = What's the weather like?", audio: "n2u03-v06" },
      { fr: "pleuvoir", en: "to rain", tip: "Only used as 'il': il pleut, il pleuvait, il a plu.", audio: "n2u03-v07" },
      { fr: "neiger", en: "to snow", tip: "Also 'il' only: il neige, il neigeait.", audio: "n2u03-v08" },
      { fr: "il y avait", en: "there was, there were", tip: "Imparfait of 'il y a' — for describing the past.", audio: "n2u03-v09" },
      { fr: "jouer", en: "to play", audio: "n2u03-v10" },
      { fr: "dehors", en: "outside", audio: "n2u03-v11" },
      { fr: "la télé", en: "the TV", audio: "n2u03-v12" }
    ],
    phrases: [
      { fr: "Quand j'étais petit, je jouais dehors.", en: "When I was little, I played outside.", audio: "n2u03-p01" },
      { fr: "Il pleuvait chaque jour.", en: "It rained every day. (it was raining day after day)", audio: "n2u03-p02" },
      { fr: "Nous mangions souvent ensemble.", en: "We often ate together.", audio: "n2u03-p03" },
      { fr: "Il y avait beaucoup de neige.", en: "There was a lot of snow.", audio: "n2u03-p04" },
      { fr: "D'habitude, je finissais tôt.", en: "Usually, I finished early.", audio: "n2u03-p05" },
      { fr: "Autrefois, il neigeait souvent ici.", en: "In the past, it often snowed here.", audio: "n2u03-p06" },
      { fr: "Pendant le week-end, nous restions à la maison.", en: "During the weekend, we stayed home.", audio: "n2u03-p07" },
      { fr: "Il pleuvait et il neigeait !", en: "It was raining and snowing!", audio: "n2u03-p08" }
    ],
    pronunciation: {
      title: "The imparfait endings all sound alike — listen close",
      points: [
        "Je parlais, tu parlais, il parlait, ils parlaient are pronounced IDENTICALLY: par-LAY. Only spelling shows the difference — a favorite dictation trap.",
        "The plural forms stand out: nous parlions (par-lee-ON), vous parliez (par-lee-AY). Hear the 'ee' glide.",
        "Être is the one rebel: j'étais, tu étais — the stem is ét-, not es-."
      ]
    },
    grammar: {
      title: "Imparfait: 'nous' stem + endings — vs passé composé",
      explanation: "Take the present-tense 'nous' form, drop -ons: nous parlons → parl-. Add -ais, -ais, -ait, -ions, -iez, -aient. (Only être is irregular: ét-.) Use the imparfait for habits (chaque jour), descriptions, and background action. Use the passé composé for single finished events that interrupt: 'Il pleuvait (background) quand je suis sorti (event).'",
      examples: [
        { fr: "Je parlais avec mon ami quand tu es arrivé.", en: "I was talking with my friend when you arrived.", audio: "n2u03-g01" },
        { fr: "Chaque dimanche, nous allions en ville.", en: "Every Sunday, we went downtown. (habit)", audio: "n2u03-g02" },
        { fr: "Quand j'étais petit, je jouais dehors chaque jour.", en: "When I was little, I played outside every day.", audio: "n2u03-g03" },
        { fr: "Hier, il a plu.", en: "Yesterday, it rained. (one finished event)", audio: "n2u03-g04" }
      ]
    },
    dialogue: {
      title: "Quand nous étions petits",
      lines: [
        { speaker: "A", fr: "Quand tu étais petit, tu jouais dehors ?", en: "When you were little, did you play outside?", audio: "n2u03-d01" },
        { speaker: "B", fr: "Oui ! Je jouais dehors chaque jour avec mes amis.", en: "Yes! I played outside every day with my friends.", audio: "n2u03-d02" },
        { speaker: "A", fr: "Moi aussi ! Et il pleuvait souvent.", en: "Me too! And it often rained.", audio: "n2u03-d03" },
        { speaker: "B", fr: "Oui, et nous restions à la maison.", en: "Yes, and we stayed home.", audio: "n2u03-d04" },
        { speaker: "A", fr: "Qu'est-ce que tu faisais à la maison ?", en: "What did you use to do at home?", audio: "n2u03-d05" },
        { speaker: "B", fr: "Je regardais la télé et je jouais avec mon frère.", en: "I watched TV and played with my brother.", audio: "n2u03-d06" },
        { speaker: "A", fr: "Moi, je jouais et je mangeais beaucoup !", en: "I played and ate a lot!", audio: "n2u03-d07" },
        { speaker: "B", fr: "Un jour, il a beaucoup neigé !", en: "One day, it snowed a lot! (single event)", audio: "n2u03-d08" },
        { speaker: "A", fr: "Et après ?", en: "And then?", audio: "n2u03-d09" },
        { speaker: "B", fr: "Nous avons joué dehors !", en: "We played outside!", audio: "n2u03-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose the imparfait of 'parler' with 'nous'.", choices: ["nous parlions", "nous avons parlé", "nous parlerons", "nous parlé"], answer: 0, explain: "Nous parlons → stem parl- + -ions = nous parlions." },
      { type: "choice", q: "Which sentence uses the tenses correctly?", choices: ["Il pleuvait quand je suis sorti.", "Il a plu quand je sortais.", "Il pleuvait quand j'ai sorti.", "Il plu quand je suis sorti."], answer: 0, explain: "Background rain = imparfait (il pleuvait); the single event of going out = passé composé (je suis sorti)." },
      { type: "choice", q: "Complete: Chaque jour, elle ___ à huit heures. (habit)", choices: ["arrivait", "est arrivée", "a arrivé", "arrive"], answer: 0, explain: "'Chaque jour' signals a repeated habit → imparfait: elle arrivait." },
      { type: "choice", q: "What is the imparfait stem of 'finir'?", choices: ["finiss-", "fini-", "fin-", "finir-"], answer: 0, explain: "Nous finissons → drop -ons → finiss-. So: je finissais." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u03-q05", transcript: "Il pleuvait quand je suis sorti.", choices: ["It was raining when I went out.", "It rained after I went out.", "I went out because it is raining.", "It is raining right now."], answer: 0, explain: "Il pleuvait (background, imparfait) quand je suis sorti (single event, passé composé)." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u03-q06", transcript: "Nous parlions et ils sont arrivés.", choices: ["We were talking and they arrived.", "We talked and they were arriving.", "We are talking and they are arriving.", "We had talked long before they arrived."], answer: 0, explain: "Nous parlions (ongoing background) et ils sont arrivés (interrupting event)." }
    ]
  },
  {
    id: "n2u04", level: 2,
    title: "Le futur", titleFr: "Le futur proche et le futur simple",
    icon: "🔮", color: "#FF6B9D",
    objectives: [
      "Use the futur proche (aller + infinitive) for near, certain plans.",
      "Form the futur simple: infinitive + -ai, -as, -a, -ons, -ez, -ont.",
      "Talk about weekend plans, projects, and next year."
    ],
    imagePrompt: "flat vector illustration of a person looking at a wall calendar with future dates circled and a small rocket launching",
    vocab: [
      { fr: "un projet", en: "a project, a plan", audio: "n2u04-v01" },
      { fr: "prévoir", en: "to plan, to foresee", tip: "Je prévois = I plan.", audio: "n2u04-v02" },
      { fr: "bientôt", en: "soon", audio: "n2u04-v03" },
      { fr: "demain", en: "tomorrow", tip: "Nasal sound: duh-MAN.", audio: "n2u04-v04" },
      { fr: "la semaine prochaine", en: "next week", audio: "n2u04-v05" },
      { fr: "l'année prochaine", en: "next year", audio: "n2u04-v06" },
      { fr: "plus tard", en: "later", audio: "n2u04-v07" },
      { fr: "décider", en: "to decide", audio: "n2u04-v08" },
      { fr: "espérer", en: "to hope", audio: "n2u04-v09" },
      { fr: "réussir", en: "to succeed", audio: "n2u04-v10" },
      { fr: "un rêve", en: "a dream", audio: "n2u04-v11" },
      { fr: "l'avenir", en: "the future", audio: "n2u04-v12" }
    ],
    phrases: [
      { fr: "Ce soir, je vais manger avec des amis.", en: "Tonight, I'm going to eat with friends.", audio: "n2u04-p01" },
      { fr: "Nous allons partir demain.", en: "We are going to leave tomorrow.", audio: "n2u04-p02" },
      { fr: "Elle va arriver bientôt.", en: "She is going to arrive soon.", audio: "n2u04-p03" },
      { fr: "Qu'est-ce que tu vas faire ?", en: "What are you going to do?", audio: "n2u04-p04" },
      { fr: "Je partirai en juillet.", en: "I will leave in July.", audio: "n2u04-p05" },
      { fr: "Nous réussirons ensemble.", en: "We will succeed together.", audio: "n2u04-p06" },
      { fr: "Tu viendras avec nous ?", en: "Will you come with us?", audio: "n2u04-p07" },
      { fr: "J'espère réussir.", en: "I hope to succeed.", audio: "n2u04-p08" }
    ],
    pronunciation: {
      title: "Future endings: the R you must pronounce",
      points: [
        "The futur simple has a hard, pronounced R: je parlerai = par-luh-RAY. Don't swallow it.",
        "'Je vais parler' (near future) vs 'je parlerai' (simple future): the first has two separate words, the second is one word ending in -ai. The exam will test your ear on this.",
        "Demain is nasal: the 'ain' is one nose sound, duh-MAN — never duh-mine."
      ]
    },
    grammar: {
      title: "Futur proche vs futur simple",
      explanation: "Futur proche = present of aller + infinitive (je vais manger) — for soon, certain plans. Futur simple = infinitive + -ai, -as, -a, -ons, -ez, -ont (je parlerai) — for later or less certain events. For -re verbs, drop the final e: prendre → prendrai. Memorize the irregular stems: être → ser-, avoir → aur-, aller → ir-, faire → fer-, venir → viendr-, vouloir → voudr-, pouvoir → pourr-.",
      examples: [
        { fr: "Je vais manger avec toi demain.", en: "I'm going to eat with you tomorrow. (near, certain)", audio: "n2u04-g01" },
        { fr: "Nous partirons en juillet.", en: "We will leave in July. (simple future)", audio: "n2u04-g02" },
        { fr: "Elle aura un nouveau projet.", en: "She will have a new project. (avoir → aur-)", audio: "n2u04-g03" },
        { fr: "Ils viendront demain.", en: "They will come tomorrow. (venir → viendr-)", audio: "n2u04-g04" }
      ]
    },
    dialogue: {
      title: "Les projets du week-end",
      lines: [
        { speaker: "A", fr: "Qu'est-ce que tu vas faire ce week-end ?", en: "What are you going to do this weekend?", audio: "n2u04-d01" },
        { speaker: "B", fr: "Samedi, je vais manger avec ma famille. Et toi ?", en: "On Saturday, I'm going to eat with my family. And you?", audio: "n2u04-d02" },
        { speaker: "A", fr: "Moi, je vais aller en ville avec mes amis.", en: "Me, I'm going to go downtown with my friends.", audio: "n2u04-d03" },
        { speaker: "B", fr: "Et dimanche ?", en: "And Sunday?", audio: "n2u04-d04" },
        { speaker: "A", fr: "Dimanche, je resterai à la maison.", en: "On Sunday, I will stay home.", audio: "n2u04-d05" },
        { speaker: "B", fr: "L'année prochaine, j'irai au Canada.", en: "Next year, I will go to Canada.", audio: "n2u04-d06" },
        { speaker: "A", fr: "Super ! Tu parleras français chaque jour !", en: "Great! You will speak French every day!", audio: "n2u04-d07" },
        { speaker: "B", fr: "Oui ! J'espère réussir.", en: "Yes! I hope to succeed.", audio: "n2u04-d08" },
        { speaker: "A", fr: "Moi aussi, j'ai un projet : je vais inviter mes amis.", en: "Me too, I have a plan: I'm going to invite my friends.", audio: "n2u04-d09" },
        { speaker: "B", fr: "Super ! On va bien manger !", en: "Great! We're going to eat well!", audio: "n2u04-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose the futur simple of 'parler' with 'je'.", choices: ["je parlerai", "je vais parler", "je parlerais", "j'ai parlé"], answer: 0, explain: "Futur simple = infinitive + -ai: je parlerai. 'Je parlerais' is the conditional — a different tense. 'Je vais parler' is futur proche." },
      { type: "choice", q: "Which sentence is futur proche?", choices: ["Je vais manger.", "Je mangerai.", "J'ai mangé.", "Je mange."], answer: 0, explain: "Aller + infinitive = futur proche: je vais manger." },
      { type: "choice", q: "Complete with the FUTUR SIMPLE: Nous ___ en juillet. (partir)", choices: ["partirons", "partons", "allons partir", "sommes partis"], answer: 0, explain: "Futur simple of partir: nous partirons. 'Allons partir' is futur proche — read the instruction!" },
      { type: "choice", q: "What is the future stem of 'être'?", choices: ["ser-", "ét-", "soi-", "êt-"], answer: 0, explain: "Être → ser-: je serai, tu seras. Irregular — memorize it." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u04-q05", transcript: "Je vais partir demain.", choices: ["I'm going to leave tomorrow.", "I will leave next year.", "I left yesterday.", "I am leaving right now."], answer: 0, explain: "Je vais partir = futur proche (aller + infinitive): I'm going to leave tomorrow." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u04-q06", transcript: "Nous réussirons ensemble.", choices: ["We will succeed together.", "We are going to succeed right now.", "We succeeded yesterday.", "We succeed every day."], answer: 0, explain: "Nous réussirons = futur simple (-ons ending): We will succeed together." }
    ]
  },
  {
    id: "n2u05", level: 2,
    title: "Les pronoms", titleFr: "Les pronoms compléments",
    icon: "👉", color: "#FFA502",
    objectives: [
      "Replace nouns with le, la, les (direct) and lui, leur (indirect).",
      "Place pronouns BEFORE the verb — including before the auxiliary in passé composé.",
      "Meet y and en: j'y vais, j'en veux."
    ],
    imagePrompt: "flat vector illustration of speech bubbles with arrows replacing nouns with the pronouns le la les lui leur",
    vocab: [
      { fr: "téléphoner à", en: "to call, to phone", tip: "Takes à → use lui/leur: je lui téléphone.", audio: "n2u05-v01" },
      { fr: "parler à", en: "to speak to", tip: "Takes à → lui/leur.", audio: "n2u05-v02" },
      { fr: "donner à", en: "to give to", audio: "n2u05-v03" },
      { fr: "demander à", en: "to ask", tip: "Demander à Marie → lui demander.", audio: "n2u05-v04" },
      { fr: "répondre à", en: "to answer", audio: "n2u05-v05" },
      { fr: "un cadeau", en: "a gift", audio: "n2u05-v06" },
      { fr: "un conseil", en: "a piece of advice", audio: "n2u05-v07" },
      { fr: "aider", en: "to help", audio: "n2u05-v08" },
      { fr: "attendre", en: "to wait for", tip: "No preposition in French: j'attends Marie.", audio: "n2u05-v09" },
      { fr: "connaître", en: "to know (a person)", tip: "Je connais, tu connais, il connaît.", audio: "n2u05-v10" },
      { fr: "un secret", en: "a secret", audio: "n2u05-v11" },
      { fr: "remercier", en: "to thank", audio: "n2u05-v12" }
    ],
    phrases: [
      { fr: "Je le vois.", en: "I see him / I see it.", audio: "n2u05-p01" },
      { fr: "Elle la regarde.", en: "She is watching it / her.", audio: "n2u05-p02" },
      { fr: "Nous les aimons.", en: "We like them.", audio: "n2u05-p03" },
      { fr: "Je lui parle.", en: "I am talking to him / to her.", audio: "n2u05-p04" },
      { fr: "Tu leur as téléphoné ?", en: "Did you call them?", audio: "n2u05-p05" },
      { fr: "Je l'ai fini.", en: "I finished it.", audio: "n2u05-p06" },
      { fr: "Il m'aide souvent.", en: "He helps me often.", audio: "n2u05-p07" },
      { fr: "On y va !", en: "Let's go!", audio: "n2u05-p08" }
    ],
    pronunciation: {
      title: "Liaisons and contractions with pronouns",
      points: [
        "Before a vowel, le/la become l': je l'ai vu, elle l'aime. One smooth sound — never 'je le ai'.",
        "Les + vowel makes a liaison: les‿amis = lay-zah-MEE. That z is mandatory.",
        "Don't confuse leur (to them, pronoun) with leurs (their, possessive plural): je leur parle vs leurs amis."
      ]
    },
    grammar: {
      title: "Object pronouns go BEFORE the verb",
      explanation: "Direct objects (le, la, les = him/her/it/them) and indirect objects (lui, leur = to him/her, to them) go directly BEFORE the verb: je le vois, je lui parle. In the passé composé, the pronoun jumps before the auxiliary: je l'ai vu. Exam bonus: with avoir, the participle agrees with a preceding DIRECT object — je les ai vus. Light intro: y replaces à + place (j'y vais = I'm going there), en replaces de + thing (j'en veux = I want some).",
      examples: [
        { fr: "Je le regarde chaque jour.", en: "I watch it every day.", audio: "n2u05-g01" },
        { fr: "Elle lui a téléphoné hier.", en: "She called him yesterday. / She called her yesterday.", audio: "n2u05-g02" },
        { fr: "Je les ai vus hier.", en: "I saw them yesterday. (agreement with preceding les)", audio: "n2u05-g03" },
        { fr: "Tu y vas demain ?", en: "Are you going there tomorrow?", audio: "n2u05-g04" }
      ]
    },
    dialogue: {
      title: "Un cadeau pour Marie",
      lines: [
        { speaker: "A", fr: "Tu as vu Marie hier ?", en: "Did you see Marie yesterday?", audio: "n2u05-d01" },
        { speaker: "B", fr: "Oui, je l'ai vue à midi.", en: "Yes, I saw her at noon.", audio: "n2u05-d02" },
        { speaker: "A", fr: "Tu lui as donné le cadeau ?", en: "Did you give her the gift?", audio: "n2u05-d03" },
        { speaker: "B", fr: "Oui, je lui ai donné le cadeau.", en: "Yes, I gave her the gift.", audio: "n2u05-d04" },
        { speaker: "A", fr: "Elle l'a aimé ?", en: "Did she like it?", audio: "n2u05-d05" },
        { speaker: "B", fr: "Oui, elle l'a beaucoup aimé !", en: "Yes, she liked it a lot!", audio: "n2u05-d06" },
        { speaker: "A", fr: "Et ses parents ? Tu leur as parlé ?", en: "And her parents? Did you talk to them?", audio: "n2u05-d07" },
        { speaker: "B", fr: "Non. Je vais leur téléphoner demain.", en: "No. I'm going to call them tomorrow.", audio: "n2u05-d08" },
        { speaker: "A", fr: "Super !", en: "Great!", audio: "n2u05-d09" },
        { speaker: "B", fr: "Oui, j'espère !", en: "Yes, I hope so!", audio: "n2u05-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Replace 'le film': Je ___ regarde.", choices: ["le", "lui", "les", "y"], answer: 0, explain: "'Le film' is a direct object, masculine singular → le. Lui is for people with à." },
      { type: "choice", q: "Replace 'à Marie': Je ___ parle.", choices: ["lui", "la", "le", "les"], answer: 0, explain: "Parler à + person → indirect object → lui (to him/her). La/le are direct." },
      { type: "choice", q: "Choose the correct word order.", choices: ["Je l'ai vu.", "J'ai le vu.", "Je vu l'ai.", "J'ai vu le."], answer: 0, explain: "The pronoun goes BEFORE the auxiliary: je l'ai vu." },
      { type: "choice", q: "Marie est partie. Je ___ ai vue hier. (agreement!)", choices: ["l'", "la", "le", "lui"], answer: 0, explain: "Before a vowel, la becomes l'. And 'vue' agrees because the direct object (Marie, feminine) comes before the verb." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u05-q05", transcript: "Je lui ai téléphoné hier.", choices: ["I called him/her yesterday.", "I called them yesterday.", "I saw him/her yesterday.", "I will call him/her tomorrow."], answer: 0, explain: "Lui = to him/her (singular). Leur would be 'them'. Passé composé → yesterday." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u05-q06", transcript: "Nous les avons vus.", choices: ["We saw them (masculine).", "We saw her.", "We see them right now.", "We have seen it."], answer: 0, explain: "Les = them (plural direct object). Passé composé → past: We saw them." }
    ]
  },
  {
    id: "n2u06", level: 2,
    title: "Au travail", titleFr: "Au travail — les verbes pronominaux",
    icon: "💼", color: "#2ED573",
    objectives: [
      "Talk about professions, the office, and your daily routine.",
      "Conjugate reflexive verbs: je me lève, tu te lèves, ils se lèvent.",
      "Use reflexives in the passé composé with être — and make them agree."
    ],
    imagePrompt: "flat vector illustration of a daily routine timeline showing waking up, working at a desk, eating, and sleeping",
    vocab: [
      { fr: "un bureau", en: "an office, a desk", audio: "n2u06-v01" },
      { fr: "un collègue", en: "a colleague, a coworker", audio: "n2u06-v02" },
      { fr: "un patron", en: "a boss", tip: "Une patronne for a female boss.", audio: "n2u06-v03" },
      { fr: "un rendez-vous", en: "an appointment, a meeting", audio: "n2u06-v04" },
      { fr: "se lever", en: "to get up", audio: "n2u06-v05" },
      { fr: "se laver", en: "to wash (oneself)", audio: "n2u06-v06" },
      { fr: "s'habiller", en: "to get dressed", audio: "n2u06-v07" },
      { fr: "se coucher", en: "to go to bed", audio: "n2u06-v08" },
      { fr: "se reposer", en: "to rest", audio: "n2u06-v09" },
      { fr: "travailler", en: "to work", audio: "n2u06-v10" },
      { fr: "à l'heure", en: "on time", audio: "n2u06-v11" },
      { fr: "en retard", en: "late (for something)", tip: "Être en retard = to be late.", audio: "n2u06-v12" }
    ],
    phrases: [
      { fr: "Je me lève à sept heures.", en: "I get up at seven o'clock.", audio: "n2u06-p01" },
      { fr: "Il se lève tôt.", en: "He gets up early.", audio: "n2u06-p02" },
      { fr: "Nous nous reposons le dimanche.", en: "We rest on Sundays.", audio: "n2u06-p03" },
      { fr: "Je travaille dans un bureau.", en: "I work in an office.", audio: "n2u06-p04" },
      { fr: "Mon collègue est sympa.", en: "My colleague is nice.", audio: "n2u06-p05" },
      { fr: "J'ai rendez-vous à dix heures.", en: "I have an appointment at ten.", audio: "n2u06-p06" },
      { fr: "Elle s'est levée tard hier.", en: "She got up late yesterday.", audio: "n2u06-p07" },
      { fr: "On se voit demain !", en: "See you tomorrow!", audio: "n2u06-p08" }
    ],
    pronunciation: {
      title: "The dropped E in reflexive pronouns",
      points: [
        "Me, te, se are soft and quick: 'je me lève' = zhuh muh lev. Don't stress the pronoun.",
        "Before a vowel, se becomes s': elle s'habille, il s'appelle. Glide it together.",
        "In 'nous nous levons', pronounce BOTH 'nous' clearly — beginners swallow the second one."
      ]
    },
    grammar: {
      title: "Reflexive verbs: je me lève, tu te lèves…",
      explanation: "Reflexive verbs add a pronoun before the verb: me, te, se, nous, vous, se. Je me lève, tu te lèves, il/elle se lève, nous nous levons, vous vous levez, ils/elles se lèvent. In the passé composé they ALWAYS use être: je me suis levé(e) — and the participle agrees with the subject, just like other être verbs.",
      examples: [
        { fr: "Je me lève à sept heures.", en: "I get up at seven o'clock.", audio: "n2u06-g01" },
        { fr: "Tu te couches tard ?", en: "Do you go to bed late?", audio: "n2u06-g02" },
        { fr: "Elle s'est levée à sept heures.", en: "She got up at seven o'clock.", audio: "n2u06-g03" },
        { fr: "Ils se sont reposés dimanche.", en: "They rested on Sunday.", audio: "n2u06-g04" }
      ]
    },
    dialogue: {
      title: "La routine de Karim",
      lines: [
        { speaker: "A", fr: "Karim, tu travailles où ?", en: "Karim, where do you work?", audio: "n2u06-d01" },
        { speaker: "B", fr: "Je travaille dans un bureau en ville.", en: "I work in an office downtown.", audio: "n2u06-d02" },
        { speaker: "A", fr: "Tu te lèves tôt ?", en: "Do you get up early?", audio: "n2u06-d03" },
        { speaker: "B", fr: "Oui, je me lève à six heures, je me lave et je m'habille.", en: "Yes, I get up at six, I wash up and I get dressed.", audio: "n2u06-d04" },
        { speaker: "A", fr: "Tu finis tard ?", en: "Do you finish late?", audio: "n2u06-d05" },
        { speaker: "B", fr: "Non, je finis à cinq heures et je me repose.", en: "No, I finish at five and I rest.", audio: "n2u06-d06" },
        { speaker: "A", fr: "Et hier ? Tu t'es reposé ?", en: "And yesterday? Did you rest?", audio: "n2u06-d07" },
        { speaker: "B", fr: "Hier, je me suis levé tôt et j'ai travaillé.", en: "Yesterday, I got up early and I worked.", audio: "n2u06-d08" },
        { speaker: "A", fr: "Moi aussi ! Je me suis couché tard.", en: "Me too! I went to bed late.", audio: "n2u06-d09" },
        { speaker: "B", fr: "On se voit demain au bureau !", en: "See you tomorrow at the office!", audio: "n2u06-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose: Je ___ lève à sept heures.", choices: ["me", "m'", "se", "te"], answer: 0, explain: "With je, the reflexive pronoun is me: je me lève." },
      { type: "choice", q: "Passé composé: Elle ___ levée tard.", choices: ["s'est", "a", "s'a", "est"], answer: 0, explain: "Reflexive verbs use être: elle s'est levée. The reflexive pronoun stays: s'est." },
      { type: "choice", q: "Which sentence is correct?", choices: ["Ils se sont reposés.", "Ils ont se reposé.", "Ils se ont reposés.", "Ils sont se reposé."], answer: 0, explain: "Être + agreement with masculine plural subject: ils se sont reposés." },
      { type: "choice", q: "Choose: Nous ___ couchons tôt.", choices: ["nous", "vous", "se", "me"], answer: 0, explain: "With nous, the reflexive pronoun is nous: nous nous couchons." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u06-q05", transcript: "Je me suis levé à six heures.", choices: ["I got up at six.", "I go to bed at six.", "I got up late.", "I am getting up right now."], answer: 0, explain: "Je me suis levé = passé composé of se lever: I got up at six." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u06-q06", transcript: "Elle s'est levée tard hier.", choices: ["She got up late yesterday.", "She went to bed late yesterday.", "He got up late yesterday.", "She gets up late every day."], answer: 0, explain: "Elle s'est levée — feminine reflexive, passé composé: She got up late yesterday." }
    ]
  },
  {
    id: "n2u07", level: 2,
    title: "Les voyages", titleFr: "Les voyages — prépositions et questions",
    icon: "✈️", color: "#1E90FF",
    objectives: [
      "Use en/à correctly with transport: en train, à pied.",
      "Book a hotel room and navigate the airport.",
      "Ask questions three ways: intonation, est-ce que, inversion (TEF speaking A)."
    ],
    imagePrompt: "flat vector illustration of a suitcase, a train, an airplane and a small hotel building with question marks above them",
    vocab: [
      { fr: "un voyage", en: "a trip, a journey", audio: "n2u07-v01" },
      { fr: "un billet", en: "a ticket", audio: "n2u07-v02" },
      { fr: "un train", en: "a train", audio: "n2u07-v03" },
      { fr: "un avion", en: "a plane", audio: "n2u07-v04" },
      { fr: "un hôtel", en: "a hotel", tip: "The h is silent: oh-TEL.", audio: "n2u07-v05" },
      { fr: "une chambre", en: "a room (hotel/bedroom)", audio: "n2u07-v06" },
      { fr: "réserver", en: "to book, to reserve", audio: "n2u07-v07" },
      { fr: "un passeport", en: "a passport", audio: "n2u07-v08" },
      { fr: "une valise", en: "a suitcase", audio: "n2u07-v09" },
      { fr: "une arrivée", en: "an arrival", audio: "n2u07-v10" },
      { fr: "l'aéroport", en: "the airport", audio: "n2u07-v11" },
      { fr: "à pied", en: "on foot", tip: "Literally 'by foot' — uses à, not en!", audio: "n2u07-v12" }
    ],
    phrases: [
      { fr: "Je voyage en train.", en: "I travel by train.", audio: "n2u07-p01" },
      { fr: "Il va à l'aéroport en bus.", en: "He is going to the airport by bus.", audio: "n2u07-p02" },
      { fr: "Nous allons à l'hôtel à pied.", en: "We are walking to the hotel.", audio: "n2u07-p03" },
      { fr: "J'ai réservé une chambre.", en: "I booked a room.", audio: "n2u07-p04" },
      { fr: "Où est l'aéroport, s'il vous plaît ?", en: "Where is the airport, please?", audio: "n2u07-p05" },
      { fr: "Est-ce que vous avez une chambre ?", en: "Do you have a room?", audio: "n2u07-p06" },
      { fr: "À quelle heure part le train ?", en: "At what time does the train leave?", audio: "n2u07-p07" },
      { fr: "Vous venez d'où ?", en: "Where are you coming from?", audio: "n2u07-p08" }
    ],
    pronunciation: {
      title: "Question music: rising intonation",
      points: [
        "Yes/no questions in speech simply RISE at the end: 'Vous venez demain↗?' The music does the work of the question mark.",
        "'Est-ce que' melts into one chunk: ess-kuh. Say 'Est-ce que tu viens ?' fast and it becomes 'ess-kuh too vee-AN'.",
        "In inversion, the -t- is pronounced and links: 'Partez-vous ?' = par-tay VOO — hear that t clearly."
      ]
    },
    grammar: {
      title: "En/à with transport + three ways to ask",
      explanation: "Use EN for vehicles you ride inside: en train, en avion, en bus, en voiture. Use À for open transport or your own feet: à pied, à vélo, à moto. Three ways to ask a question: (1) raise your voice — 'Tu viens ?'; (2) add 'est-ce que' — 'Est-ce que tu viens ?'; (3) invert verb and subject — 'Viens-tu ?' For TEF speaking, (1) and (2) are plenty; inversion is formal and mostly written.",
      examples: [
        { fr: "Je vais à Paris en avion.", en: "I'm going to Paris by plane.", audio: "n2u07-g01" },
        { fr: "Elle voyage à pied.", en: "She travels on foot.", audio: "n2u07-g02" },
        { fr: "Est-ce que tu as réservé une chambre ?", en: "Did you book a room?", audio: "n2u07-g03" },
        { fr: "Partez-vous demain ?", en: "Are you leaving tomorrow? (formal inversion)", audio: "n2u07-g04" }
      ]
    },
    dialogue: {
      title: "À la réception de l'hôtel",
      lines: [
        { speaker: "A", fr: "Bonsoir ! Est-ce que vous avez une chambre pour deux jours ?", en: "Good evening! Do you have a room for two days?", audio: "n2u07-d01" },
        { speaker: "B", fr: "Oui. Vous venez d'où ?", en: "Yes. Where are you coming from?", audio: "n2u07-d02" },
        { speaker: "A", fr: "Je viens de Toronto.", en: "I'm coming from Toronto.", audio: "n2u07-d03" },
        { speaker: "B", fr: "Vous voyagez en avion ?", en: "Are you traveling by plane?", audio: "n2u07-d04" },
        { speaker: "A", fr: "Oui, et ma femme voyage en train.", en: "Yes, and my wife is traveling by train.", audio: "n2u07-d05" },
        { speaker: "B", fr: "À quelle heure arrivez-vous demain ?", en: "At what time are you arriving tomorrow?", audio: "n2u07-d06" },
        { speaker: "A", fr: "J'arrive à dix heures. Où est l'hôtel ?", en: "I'm arriving at ten. Where is the hotel?", audio: "n2u07-d07" },
        { speaker: "B", fr: "L'hôtel est en ville.", en: "The hotel is downtown.", audio: "n2u07-d08" },
        { speaker: "A", fr: "Merci ! J'ai réservé hier.", en: "Thank you! I booked yesterday.", audio: "n2u07-d09" },
        { speaker: "B", fr: "Très bien ! À demain !", en: "Very good! See you tomorrow!", audio: "n2u07-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose: Je voyage ___ train.", choices: ["en", "à", "dans", "avec"], answer: 0, explain: "Vehicles you ride inside take en: en train, en avion, en bus." },
      { type: "choice", q: "Choose: Elle va à l'école ___ pied.", choices: ["à", "en", "dans", "par"], answer: 0, explain: "À pied — on foot always uses à. The classic exception to en!" },
      { type: "choice", q: "Which question uses inversion?", choices: ["Partez-vous demain ?", "Est-ce que vous partez demain ?", "Vous partez demain ?", "Tu pars demain ?"], answer: 0, explain: "Inversion swaps verb and subject with a hyphen: Partez-vous ?" },
      { type: "choice", q: "Choose: ___ vous avez une chambre ?", choices: ["Est-ce que", "Qu'est-ce que", "Pourquoi", "Quand"], answer: 0, explain: "Est-ce que turns a statement into a yes/no question: Est-ce que vous avez une chambre ?" },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u07-q05", transcript: "Est-ce que vous avez réservé ?", choices: ["Did you book?", "Have you arrived?", "Are you leaving?", "Do you travel?"], answer: 0, explain: "Est-ce que vous avez réservé — passé composé of réserver: Did you book?" },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u07-q06", transcript: "Je voyage en avion.", choices: ["I travel by plane.", "I travel by train.", "I travel on foot.", "I travel by bus."], answer: 0, explain: "En avion — inside the vehicle → en. Not à pied, not en train." }
    ]
  },
  {
    id: "n2u08", level: 2,
    title: "La santé", titleFr: "La santé — devoir, pouvoir, vouloir",
    icon: "🏥", color: "#FF4757",
    objectives: [
      "Name body parts and describe symptoms: j'ai mal à la tête.",
      "See a doctor in French: explain what's wrong, understand advice.",
      "Use devoir, pouvoir, vouloir + il faut + infinitive correctly."
    ],
    imagePrompt: "flat vector illustration of a friendly doctor talking to a patient, with labeled body parts around them",
    vocab: [
      { fr: "la tête", en: "the head", audio: "n2u08-v01" },
      { fr: "le dos", en: "the back", tip: "The s is silent: 'doh'.", audio: "n2u08-v02" },
      { fr: "le ventre", en: "the stomach, the belly", audio: "n2u08-v03" },
      { fr: "la gorge", en: "the throat", audio: "n2u08-v04" },
      { fr: "le bras", en: "the arm", audio: "n2u08-v05" },
      { fr: "la jambe", en: "the leg", audio: "n2u08-v06" },
      { fr: "le cœur", en: "the heart", tip: "The œ is one sound: 'kuhr'.", audio: "n2u08-v07" },
      { fr: "avoir mal à", en: "to have pain in", tip: "J'ai mal à la tête = I have a headache.", audio: "n2u08-v08" },
      { fr: "la fièvre", en: "the fever", audio: "n2u08-v09" },
      { fr: "un médicament", en: "a medicine", audio: "n2u08-v10" },
      { fr: "un médecin", en: "a doctor", audio: "n2u08-v11" },
      { fr: "l'hôpital", en: "the hospital", tip: "Silent h, and the ô wears a circumflex.", audio: "n2u08-v12" }
    ],
    phrases: [
      { fr: "J'ai mal à la tête.", en: "I have a headache.", audio: "n2u08-p01" },
      { fr: "Elle a mal au dos.", en: "Her back hurts.", audio: "n2u08-p02" },
      { fr: "Je dois voir un médecin.", en: "I must see a doctor.", audio: "n2u08-p03" },
      { fr: "Tu peux attendre ici.", en: "You can wait here.", audio: "n2u08-p04" },
      { fr: "Je veux un rendez-vous.", en: "I want an appointment.", audio: "n2u08-p05" },
      { fr: "Il faut se reposer.", en: "One must rest. / You need to rest.", audio: "n2u08-p06" },
      { fr: "Vous devez vous reposer.", en: "You must rest.", audio: "n2u08-p07" },
      { fr: "Il peut venir demain.", en: "He can come tomorrow.", audio: "n2u08-p08" }
    ],
    pronunciation: {
      title: "The silent letters in body parts",
      points: [
        "Le dos: the final s is completely silent — just say 'doh'. Same for le bras: 'brah'.",
        "Le cœur: the œ ligature is ONE vowel sound, like the 'u' in 'burn': kuhr.",
        "L'hôpital: the h is silent (loh-pee-TAL), and the circumflex on ô is a hint that an old s disappeared."
      ]
    },
    grammar: {
      title: "Devoir, pouvoir, vouloir + il faut + infinitive",
      explanation: "These three modal verbs are followed DIRECTLY by an infinitive — no preposition: je dois partir (I must leave), tu peux venir (you can come), elle veut manger (she wants to eat). They're irregular: je dois / tu dois / il doit / nous devons; je peux / tu peux / il peut / nous pouvons; je veux / tu veux / il veut / nous voulons / vous voulez / ils veulent. 'Il faut + infinitive' means 'it's necessary to': il faut travailler.",
      examples: [
        { fr: "Je dois voir un médecin.", en: "I must see a doctor.", audio: "n2u08-g01" },
        { fr: "Tu peux attendre ici.", en: "You can wait here.", audio: "n2u08-g02" },
        { fr: "Elle veut se reposer.", en: "She wants to rest.", audio: "n2u08-g03" },
        { fr: "Il faut travailler.", en: "One has to work. / It's necessary to work.", audio: "n2u08-g04" }
      ]
    },
    dialogue: {
      title: "Chez le médecin",
      lines: [
        { speaker: "A", fr: "Bonjour !", en: "Hello!", audio: "n2u08-d01" },
        { speaker: "B", fr: "Bonjour ! Où avez-vous mal ?", en: "Hello! Where does it hurt?", audio: "n2u08-d02" },
        { speaker: "A", fr: "J'ai mal à la tête et j'ai mal à la gorge.", en: "My head hurts and my throat hurts.", audio: "n2u08-d03" },
        { speaker: "B", fr: "Vous avez de la fièvre ?", en: "Do you have a fever?", audio: "n2u08-d04" },
        { speaker: "A", fr: "Oui, un peu.", en: "Yes, a little.", audio: "n2u08-d05" },
        { speaker: "B", fr: "Vous devez vous reposer trois jours.", en: "You must rest for three days.", audio: "n2u08-d06" },
        { speaker: "A", fr: "Je peux travailler ?", en: "Can I work?", audio: "n2u08-d07" },
        { speaker: "B", fr: "Non, il faut rester à la maison.", en: "No, you have to stay home.", audio: "n2u08-d08" },
        { speaker: "A", fr: "Je peux prendre un médicament ?", en: "Can I take a medicine?", audio: "n2u08-d09" },
        { speaker: "B", fr: "Oui, vous pouvez.", en: "Yes, you can.", audio: "n2u08-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose: Je ___ partir. (devoir, je)", choices: ["dois", "devons", "doit", "devoir"], answer: 0, explain: "Devoir with je: je dois. Then the infinitive directly: je dois partir." },
      { type: "choice", q: "Choose: Tu ___ venir demain. (pouvoir, tu)", choices: ["peux", "pouvoir", "peut", "peux-tu"], answer: 0, explain: "Pouvoir with tu: tu peux. Tu peux venir demain." },
      { type: "choice", q: "Which sentence is correct?", choices: ["Il faut travailler.", "Il faut travaille.", "Il faut à travailler.", "Il faut de travailler."], answer: 0, explain: "Il faut is followed directly by the infinitive — no preposition: il faut travailler." },
      { type: "choice", q: "Choose: Elle ___ se reposer. (vouloir, elle)", choices: ["veut", "vouloir", "veux", "veulent"], answer: 0, explain: "Vouloir with elle: elle veut. Elle veut se reposer." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u08-q05", transcript: "J'ai mal à la tête.", choices: ["I have a headache.", "I have a stomachache.", "My head is fine.", "I have a cold."], answer: 0, explain: "Avoir mal à la tête = to have a headache." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u08-q06", transcript: "Vous devez vous reposer.", choices: ["You must rest.", "You can work.", "You want to leave.", "You must eat."], answer: 0, explain: "Vous devez = you must (devoir). Vous devez vous reposer: You must rest." }
    ]
  },
  {
    id: "n2u09", level: 2,
    title: "Donner son avis", titleFr: "Donner son avis — l'opinion",
    icon: "💭", color: "#A55EEA",
    objectives: [
      "Give opinions: à mon avis, je pense que, selon moi.",
      "Justify with parce que + sentence and pour + infinitive.",
      "Structure a TEF speaking section B answer: opinion + reason + example."
    ],
    imagePrompt: "flat vector illustration of two people having a polite debate, speech bubbles showing à mon avis and parce que",
    vocab: [
      { fr: "un avis", en: "an opinion", tip: "À mon avis = in my opinion.", audio: "n2u09-v01" },
      { fr: "penser", en: "to think", tip: "Je pense que = I think that.", audio: "n2u09-v02" },
      { fr: "croire", en: "to believe", audio: "n2u09-v03" },
      { fr: "trouver", en: "to find; to consider", tip: "Je trouve ça bien = I think it's good.", audio: "n2u09-v04" },
      { fr: "d'accord", en: "in agreement", tip: "Je suis d'accord = I agree.", audio: "n2u09-v05" },
      { fr: "pas d'accord", en: "not in agreement", tip: "Je ne suis pas d'accord = I disagree.", audio: "n2u09-v06" },
      { fr: "avoir raison", en: "to be right", tip: "Literally 'to have reason'.", audio: "n2u09-v07" },
      { fr: "avoir tort", en: "to be wrong", tip: "Literally 'to have wrong'.", audio: "n2u09-v08" },
      { fr: "important", en: "important", audio: "n2u09-v09" },
      { fr: "intéressant", en: "interesting", audio: "n2u09-v10" },
      { fr: "difficile", en: "difficult", audio: "n2u09-v11" },
      { fr: "le problème", en: "the problem", audio: "n2u09-v12" }
    ],
    phrases: [
      { fr: "À mon avis, c'est important.", en: "In my opinion, it's important.", audio: "n2u09-p01" },
      { fr: "Je pense que c'est bien.", en: "I think it's good.", audio: "n2u09-p02" },
      { fr: "Selon moi, il faut travailler.", en: "In my view, one must work.", audio: "n2u09-p03" },
      { fr: "Je suis d'accord avec toi.", en: "I agree with you.", audio: "n2u09-p04" },
      { fr: "Tu as raison !", en: "You're right!", audio: "n2u09-p05" },
      { fr: "Parce que c'est important.", en: "Because it's important.", audio: "n2u09-p06" },
      { fr: "Pour réussir, il faut travailler.", en: "To succeed, one must work.", audio: "n2u09-p07" },
      { fr: "Je crois que oui.", en: "I think so.", audio: "n2u09-p08" }
    ],
    pronunciation: {
      title: "Sounding convinced, not robotic",
      points: [
        "In 'je pense que', the 'que' is a soft, quick 'kuh' — don't punch it: zhuh ponss-kuh.",
        "'Parce que' runs together into two beats: parss-kuh. Native speakers never separate it.",
        "'D'accord' links the d': dah-KOR. That little apostrophe is a bridge, not a wall."
      ]
    },
    grammar: {
      title: "Opinion + justification: parce que / pour + infinitive",
      explanation: "Give your opinion with 'à mon avis', 'je pense que', or 'selon moi'. Then ALWAYS justify it — an unjustified opinion scores nothing on the TEF. Use 'parce que + sentence' (because) or 'pour + infinitive' (in order to). The winning structure for TEF speaking section B is: opinion + reason + example.",
      examples: [
        { fr: "À mon avis, le français est important parce que je veux travailler au Canada.", en: "In my opinion, French is important because I want to work in Canada.", audio: "n2u09-g01" },
        { fr: "Je pense que tu as raison.", en: "I think you're right.", audio: "n2u09-g02" },
        { fr: "Selon moi, il faut parler français pour trouver un travail.", en: "In my view, one must speak French to find a job.", audio: "n2u09-g03" },
        { fr: "Je suis d'accord avec toi parce que tu as raison.", en: "I agree with you because you're right.", audio: "n2u09-g04" }
      ]
    },
    dialogue: {
      title: "Faut-il parler français au Canada ?",
      lines: [
        { speaker: "A", fr: "À mon avis, il faut parler français au Canada.", en: "In my opinion, one must speak French in Canada.", audio: "n2u09-d01" },
        { speaker: "B", fr: "Pourquoi ?", en: "Why?", audio: "n2u09-d02" },
        { speaker: "A", fr: "Parce que c'est important pour trouver un travail.", en: "Because it's important for finding a job.", audio: "n2u09-d03" },
        { speaker: "B", fr: "Moi, je pense que c'est difficile.", en: "Me, I think it's difficult.", audio: "n2u09-d04" },
        { speaker: "A", fr: "Je suis d'accord, c'est difficile. Mais c'est important !", en: "I agree, it's difficult. But it's important!", audio: "n2u09-d05" },
        { speaker: "B", fr: "Pourquoi c'est important ?", en: "Why is it important?", audio: "n2u09-d06" },
        { speaker: "A", fr: "Parce que je veux travailler au Canada et parler avec mes collègues.", en: "Because I want to work in Canada and talk with my colleagues.", audio: "n2u09-d07" },
        { speaker: "B", fr: "Tu as raison !", en: "You're right!", audio: "n2u09-d08" },
        { speaker: "A", fr: "Alors, on parle français ensemble ?", en: "So, shall we speak French together?", audio: "n2u09-d09" },
        { speaker: "B", fr: "D'accord ! Je suis d'accord avec toi !", en: "Agreed! I agree with you!", audio: "n2u09-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Complete: À mon avis, il faut travailler ___ réussir.", choices: ["pour", "parce que", "à", "de"], answer: 0, explain: "'Pour + infinitive' expresses purpose: pour réussir = in order to succeed. 'Parce que' needs a full sentence after it." },
      { type: "choice", q: "What is the BEST structure for TEF speaking section B?", choices: ["Opinion + reason + example", "Just the opinion, nothing more", "Only questions, no opinion", "A list of random words"], answer: 0, explain: "Section B asks you to convince someone. Opinion alone scores little — always add parce que/pour and an example." },
      { type: "choice", q: "Complete: Je pense ___ tu as raison.", choices: ["que", "à", "de", "pour"], answer: 0, explain: "Je pense QUE + sentence. No preposition between them." },
      { type: "choice", q: "Complete: ___ moi, c'est important.", choices: ["Selon", "À mon", "Pour", "Avec"], answer: 0, explain: "Selon moi = in my view. (À mon avis is the other fixed form — never 'à mon moi'.)" },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u09-q05", transcript: "À mon avis, c'est important parce que je veux travailler au Canada.", choices: ["In my opinion it's important because I want to work in Canada.", "In my opinion it's difficult because I work in Canada.", "I think it's important to visit Canada.", "I want to work because Canada is important."], answer: 0, explain: "À mon avis (opinion) + parce que (reason): In my opinion it's important because I want to work in Canada." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u09-q06", transcript: "Je suis d'accord avec toi.", choices: ["I agree with you.", "I disagree with you.", "I am thinking about you.", "I am talking to you."], answer: 0, explain: "Je suis d'accord avec toi = I agree with you. The TEF loves testing this against 'pas d'accord'." }
    ]
  },
  {
    id: "n2u10", level: 2,
    title: "Raconter une histoire", titleFr: "Raconter une histoire — le récit",
    icon: "📖", color: "#E17055",
    objectives: [
      "Order events with connectors: d'abord, ensuite, puis, soudain, enfin.",
      "Combine imparfait (background) + passé composé (events) to narrate.",
      "Tell a complete anecdote — the core skill for TEF writing and speaking."
    ],
    imagePrompt: "flat vector illustration of an open storybook with connected panels showing sun, rain and a traveler",
    vocab: [
      { fr: "d'abord", en: "first, at first", audio: "n2u10-v01" },
      { fr: "ensuite", en: "then, next", audio: "n2u10-v02" },
      { fr: "puis", en: "then", audio: "n2u10-v03" },
      { fr: "soudain", en: "suddenly", audio: "n2u10-v04" },
      { fr: "enfin", en: "finally, at last", audio: "n2u10-v05" },
      { fr: "tout à coup", en: "all of a sudden", audio: "n2u10-v06" },
      { fr: "à ce moment-là", en: "at that moment", audio: "n2u10-v07" },
      { fr: "une histoire", en: "a story", audio: "n2u10-v08" },
      { fr: "un souvenir", en: "a memory", audio: "n2u10-v09" },
      { fr: "raconter", en: "to tell (a story)", audio: "n2u10-v10" },
      { fr: "continuer", en: "to continue", audio: "n2u10-v11" },
      { fr: "se terminer", en: "to end", tip: "L'histoire se termine bien = The story ends well.", audio: "n2u10-v12" }
    ],
    phrases: [
      { fr: "D'abord, il faisait beau.", en: "First, the weather was nice.", audio: "n2u10-p01" },
      { fr: "Ensuite, nous sommes partis.", en: "Then, we left.", audio: "n2u10-p02" },
      { fr: "Puis il a plu.", en: "Then it rained.", audio: "n2u10-p03" },
      { fr: "Soudain, nous avons vu un ami.", en: "Suddenly, we saw a friend.", audio: "n2u10-p04" },
      { fr: "Enfin, nous sommes rentrés.", en: "Finally, we went home.", audio: "n2u10-p05" },
      { fr: "À ce moment-là, j'ai téléphoné.", en: "At that moment, I called.", audio: "n2u10-p06" },
      { fr: "Tout à coup, il a neigé.", en: "All of a sudden, it snowed.", audio: "n2u10-p07" },
      { fr: "L'histoire se termine bien.", en: "The story ends well.", audio: "n2u10-p08" }
    ],
    pronunciation: {
      title: "Storytelling rhythm",
      points: [
        "Pause AFTER connectors: 'D'abord… [beat] il faisait beau.' That pause is what makes you sound like a storyteller, not a robot.",
        "Soudain: stress the LAST syllable — soo-DAN. French stress is almost always final.",
        "Ensuite flows as one word: 'on-sweet'. Don't chop it into en-suite."
      ]
    },
    grammar: {
      title: "Narrating: imparfait for background, passé composé for events",
      explanation: "A story needs BOTH past tenses. The imparfait paints the background — the weather, what people were doing: il faisait beau, nous marchions. The passé composé moves the story forward — the events: soudain, il a plu. Connectors order everything: d'abord → ensuite / puis → soudain → enfin. This combination is exactly what TEF writing and speaking reward.",
      examples: [
        { fr: "D'abord, il faisait beau et nous étions en ville.", en: "First, the weather was nice and we were downtown.", audio: "n2u10-g01" },
        { fr: "Ensuite, nous sommes partis.", en: "Then, we left.", audio: "n2u10-g02" },
        { fr: "Soudain, il a plu et nous sommes rentrés.", en: "Suddenly, it rained and we went home.", audio: "n2u10-g03" },
        { fr: "Enfin, nous avons mangé ensemble à la maison.", en: "Finally, we ate together at home.", audio: "n2u10-g04" }
      ]
    },
    dialogue: {
      title: "Une histoire de voyage",
      lines: [
        { speaker: "A", fr: "Ton voyage, c'était bien ?", en: "Your trip — was it good?", audio: "n2u10-d01" },
        { speaker: "B", fr: "Oui ! D'abord, il faisait beau et nous étions en ville.", en: "Yes! First, the weather was nice and we were downtown.", audio: "n2u10-d02" },
        { speaker: "A", fr: "Et ensuite ?", en: "And then?", audio: "n2u10-d03" },
        { speaker: "B", fr: "Ensuite, nous sommes partis en train.", en: "Then, we left by train.", audio: "n2u10-d04" },
        { speaker: "A", fr: "Où êtes-vous allés ?", en: "Where did you go?", audio: "n2u10-d05" },
        { speaker: "B", fr: "À Québec ! Puis, soudain, il a plu.", en: "To Quebec City! Then, suddenly, it rained.", audio: "n2u10-d06" },
        { speaker: "A", fr: "Oh non ! Qu'est-ce que vous avez fait ?", en: "Oh no! What did you do?", audio: "n2u10-d07" },
        { speaker: "B", fr: "À ce moment-là, nous avons téléphoné à l'hôtel.", en: "At that moment, we called the hotel.", audio: "n2u10-d08" },
        { speaker: "A", fr: "Et après ?", en: "And after that?", audio: "n2u10-d09" },
        { speaker: "B", fr: "Enfin, nous sommes rentrés et nous avons bien mangé. C'était super !", en: "Finally, we went home and ate well. It was great!", audio: "n2u10-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "Choose the right tense: D'abord, il ___ beau. (background)", choices: ["faisait", "a fait", "fait", "fera"], answer: 0, explain: "Background description → imparfait: il faisait beau." },
      { type: "choice", q: "Complete: Soudain, nous ___ un ami. (event)", choices: ["avons vu", "voyions", "voyons", "voir"], answer: 0, explain: "'Soudain' introduces a single event that moves the story → passé composé: nous avons vu." },
      { type: "choice", q: "Which connector STARTS a story?", choices: ["D'abord", "Enfin", "Soudain", "Tout à coup"], answer: 0, explain: "D'abord opens the narration. Enfin closes it; soudain/tout à coup mark surprises in the middle." },
      { type: "choice", q: "Choose the correct narration.", choices: ["Il pleuvait quand nous sommes sortis.", "Il a plu quand nous sortions.", "Il pleuvait quand nous sortions.", "Il pleut quand nous sommes sortis."], answer: 0, explain: "Background rain (imparfait) + the event of going out (passé composé): Il pleuvait quand nous sommes sortis." },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u10-q05", transcript: "D'abord, il faisait beau. Ensuite, nous sommes partis.", choices: ["First it was nice, then we left.", "First we left, then it was nice.", "It is nice and we are leaving.", "We left because it was nice."], answer: 0, explain: "D'abord (background, imparfait) → ensuite (event, passé composé). Order matters!" },
      { type: "listening", q: "Listen and choose the correct meaning.", audio: "n2u10-q06", transcript: "Soudain, il a plu et nous sommes rentrés.", choices: ["Suddenly it rained and we went home.", "It was raining when we went home.", "We went home and then it rained.", "Suddenly we left because of the rain."], answer: 0, explain: "Soudain + two passé composé events in order: it rained, then we went home." }
    ]
  }
];