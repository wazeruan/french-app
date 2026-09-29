const N1_UNITS = [
  {
    id: "n1u01", level: 1,
    title: "The Sounds of French", titleFr: "Les sons du français",
    icon: "🔤", color: "#4A90D9",
    objectives: [
      "Pronounce the four nasal vowels (on, an/en, in, ain) so 'bon' never sounds like English 'bone'",
      "Read any French word aloud using the silent-final-letter rules",
      "Produce the French R in the throat, never rolled on the tongue"
    ],
    imagePrompt: "flat vector illustration of a human head side view showing mouth and throat with tongue position for French nasal vowels, small labels on an in",
    vocab: [
      { fr: "Bonjour", en: "Hello (daytime)", tip: "nasal 'on' — never pronounce the n", audio: "n1u01-v01" },
      { fr: "Bonsoir", en: "Good evening", tip: "used when arriving, roughly after 6pm", audio: "n1u01-v02" },
      { fr: "Salut", en: "Hi (informal)", tip: "final t is silent: 'sah-loo'", audio: "n1u01-v03" },
      { fr: "Oui", en: "Yes", audio: "n1u01-v04" },
      { fr: "Non", en: "No", tip: "nasal 'on': [nɔ̃]", audio: "n1u01-v05" },
      { fr: "Merci", en: "Thank you / thanks", tip: "throaty R, never rolled", audio: "n1u01-v06" },
      { fr: "S'il vous plaît", en: "Please (formal)", tip: "literally 'if it pleases you'", audio: "n1u01-v07" },
      { fr: "Excusez-moi", en: "Excuse me (formal)", tip: "ends 'excuse-ay mwa'", audio: "n1u01-v08" },
      { fr: "Au revoir", en: "Goodbye", tip: "the final e is barely whispered", audio: "n1u01-v09" },
      { fr: "Bonne nuit", en: "Good night", tip: "only when parting for bed, not when arriving", audio: "n1u01-v10" },
      { fr: "Pardon", en: "Sorry / pardon", tip: "nasal 'on', final n silent", audio: "n1u01-v11" },
      { fr: "Monsieur / Madame", en: "Sir / Madam", tip: "'sieu' is silent in monsieur: 'muh-syuh'", audio: "n1u01-v12" }
    ],
    phrases: [
      { fr: "Comment allez-vous ?", en: "How are you? (formal)", audio: "n1u01-p01" },
      { fr: "Ça va ?", en: "How's it going? (informal)", audio: "n1u01-p02" },
      { fr: "Très bien, merci.", en: "Very well, thanks.", audio: "n1u01-p03" },
      { fr: "Enchanté / Enchantée.", en: "Nice to meet you.", tip: "add -e if you are a woman: enchantée", audio: "n1u01-p04" },
      { fr: "À demain.", en: "See you tomorrow.", audio: "n1u01-p05" },
      { fr: "Bonne journée !", en: "Have a good day!", audio: "n1u01-p06" },
      { fr: "Je ne comprends pas.", en: "I don't understand.", tip: "fixed phrase for now — a lifesaver in exams", audio: "n1u01-p07" },
      { fr: "Parlez lentement, s'il vous plaît.", en: "Speak slowly, please.", tip: "fixed phrase for now", audio: "n1u01-p08" }
    ],
    pronunciation: {
      title: "The 3 sound rules that unlock French",
      points: [
        "Final consonants go silent: the s, t, d, p, x at the end of most words are not pronounced — 'salut' is 'sah-loo', 'beaucoup' is 'boku'.",
        "'on', 'an', 'en', 'in', 'ain' are ONE nasal sound each: push air through your nose and never fully say the n — 'bon' [bɔ̃], 'vin' [vɛ̃].",
        "The French R lives in the throat — a soft gargle, as in 'merci' and 'rue'. And the letter h is always silent: 'heure' starts with a vowel sound."
      ]
    },
    grammar: {
      title: "How French spelling maps to sound",
      explanation: "French spelling looks scary but it is very regular — more regular than English. Three rules unlock almost everything: (1) final consonants (s, t, d, p, x) are usually silent; (2) 'on', 'an', 'en', 'in', 'ain' are single nasal vowels, and the n is never fully pronounced; (3) the French R is made at the back of the throat, like a soft gargle, never rolled. Learn these and you can read any French word out loud.",
      examples: [
        { fr: "beaucoup", en: "a lot — the p is silent: [boku]", audio: "n1u01-g01" },
        { fr: "bon", en: "good — 'on' is one nasal vowel, the n is silent: [bɔ̃]", audio: "n1u01-g02" },
        { fr: "merci", en: "thanks — throaty R: [mɛʁsi]", audio: "n1u01-g03" },
        { fr: "les", en: "the (plural) — final s is silent: [le]", audio: "n1u01-g04" }
      ]
    },
    dialogue: {
      title: "First contact",
      lines: [
        { speaker: "A", fr: "Bonjour, monsieur !", en: "Hello, sir!", audio: "n1u01-d01" },
        { speaker: "B", fr: "Bonjour, madame !", en: "Hello, madam!", audio: "n1u01-d02" },
        { speaker: "A", fr: "Comment allez-vous ?", en: "How are you?", audio: "n1u01-d03" },
        { speaker: "B", fr: "Très bien, merci. Et vous ?", en: "Very well, thanks. And you?", audio: "n1u01-d04" },
        { speaker: "A", fr: "Ça va bien, merci.", en: "I'm well, thanks.", audio: "n1u01-d05" },
        { speaker: "B", fr: "Excusez-moi — parlez lentement, s'il vous plaît.", en: "Excuse me — speak slowly, please.", audio: "n1u01-d06" },
        { speaker: "A", fr: "Oui, pardon.", en: "Yes, sorry.", audio: "n1u01-d07" },
        { speaker: "B", fr: "Merci.", en: "Thanks.", audio: "n1u01-d08" },
        { speaker: "A", fr: "Au revoir, bonne journée !", en: "Goodbye, have a good day!", audio: "n1u01-d09" },
        { speaker: "B", fr: "Au revoir !", en: "Goodbye!", audio: "n1u01-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "How do you say 'Good evening' when arriving somewhere?", choices: ["Bonsoir", "Bonne nuit", "Bonjour", "Salut"], answer: 0, explain: "'Bonsoir' is for arriving in the evening; 'bonne nuit' is only when parting for bed.", audio: "n1u01-q01" },
      { type: "choice", q: "In 'beaucoup', what happens to the p?", choices: ["It's silent", "It's pronounced hard", "It becomes a b sound", "The word has no p"], answer: 0, explain: "Final consonants are usually silent — 'beaucoup' sounds like [boku].", audio: "n1u01-q02" },
      { type: "choice", q: "Which word contains a nasal vowel?", choices: ["bon", "merci", "salut", "oui"], answer: 0, explain: "'on' is a single nasal sound [bɔ̃]; merci, salut and oui have no nasal vowel.", audio: "n1u01-q03" },
      { type: "choice", q: "How do you say 'please' to a stranger?", choices: ["S'il vous plaît", "S'il te plaît", "S'il lui plaît", "S'il me plaît"], answer: 0, explain: "'vous' is the formal you — always safe with strangers.", audio: "n1u01-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u01-q05", transcript: "Bonjour, madame.", choices: ["Bonjour, madame.", "Bonsoir, madame.", "Bonjour, monsieur.", "Salut, madame."], answer: 0, explain: "Listen for bonjour vs bonsoir, and madame vs monsieur." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u01-q06", transcript: "Merci beaucoup.", choices: ["Merci beaucoup.", "Merci bien.", "Merci, madame.", "Merci beaucoup, monsieur."], answer: 0, explain: "'beaucoup' ends in a silent p: [boku] — a classic trap." }
    ]
  },
  {
    id: "n1u02", level: 1,
    title: "Hello and Goodbye", titleFr: "Bonjour !",
    icon: "🗣️", color: "#E63946",
    objectives: [
      "Choose 'tu' or 'vous' correctly in any greeting situation",
      "Greet and say goodbye naturally, formally and informally",
      "Link words with liaisons (vous‿avez, les‿amis) without pausing"
    ],
    imagePrompt: "flat vector illustration of two greeting scenes side by side, a formal bow labeled vous and friends waving labeled tu, French café street background",
    vocab: [
      { fr: "Salut", en: "Hi (informal)", tip: "friends and family only", audio: "n1u02-v01" },
      { fr: "À bientôt", en: "See you soon", audio: "n1u02-v02" },
      { fr: "À tout à l'heure", en: "See you later (today)", tip: "use when you'll meet again today", audio: "n1u02-v03" },
      { fr: "De rien", en: "You're welcome", tip: "literally 'of nothing'", audio: "n1u02-v04" },
      { fr: "Bienvenue", en: "Welcome", audio: "n1u02-v05" },
      { fr: "S'il te plaît", en: "Please (informal)", tip: "'te' = informal you", audio: "n1u02-v06" },
      { fr: "Excuse-moi", en: "Excuse me (informal)", audio: "n1u02-v07" },
      { fr: "Désolé / Désolée", en: "Sorry", tip: "add -e when a woman is speaking", audio: "n1u02-v08" },
      { fr: "Parler", en: "To speak", audio: "n1u02-v09" },
      { fr: "Répéter", en: "To repeat", audio: "n1u02-v10" },
      { fr: "Un peu", en: "A little", tip: "nasal 'un' [œ̃]", audio: "n1u02-v11" },
      { fr: "Aussi", en: "Also, too", audio: "n1u02-v12" }
    ],
    phrases: [
      { fr: "Comment vas-tu ?", en: "How are you? (informal)", audio: "n1u02-p01" },
      { fr: "Tu vas bien ?", en: "Are you well? (informal)", audio: "n1u02-p02" },
      { fr: "Je vais bien, merci.", en: "I'm well, thanks.", tip: "fixed phrase for now", audio: "n1u02-p03" },
      { fr: "Et toi ?", en: "And you? (informal)", audio: "n1u02-p04" },
      { fr: "À demain !", en: "See you tomorrow!", audio: "n1u02-p05" },
      { fr: "Passe une bonne journée !", en: "Have a good day! (informal)", audio: "n1u02-p06" },
      { fr: "De rien, avec plaisir.", en: "You're welcome, with pleasure.", audio: "n1u02-p07" }
    ],
    pronunciation: {
      title: "Sound natural: liaisons and the u / ou trap",
      points: [
        "Liaison: when a word ending in a silent consonant meets a vowel, the consonant wakes up and links — 'vous‿avez' sounds like 'voo-zah-vay', 'les‿amis' like 'lay-zah-mee'.",
        "'u' [y] and 'ou' [u] are different sounds: 'tu' (you) vs 'tout' (all). Round your lips tight and push the sound forward for 'u'.",
        "Yes/no questions rise at the end: 'Tu vas bien ?' goes UP. In listening exams, that rising tune signals a question even with no question word."
      ]
    },
    grammar: {
      title: "Tu vs vous — the two words for 'you'",
      explanation: "French has two words for 'you'. Use 'tu' with friends, family, children and people your age; use 'vous' with strangers, older people, shops and officials. The verb changes too: 'tu parles' but 'vous parlez'. When in doubt, 'vous' is always safe — nobody is ever offended by too much politeness.",
      examples: [
        { fr: "Tu parles français ?", en: "Do you speak French? — to a friend", audio: "n1u02-g01" },
        { fr: "Vous parlez français ?", en: "Do you speak French? — to a stranger", audio: "n1u02-g02" },
        { fr: "S'il te plaît, répète.", en: "Please repeat. — to a friend", audio: "n1u02-g03" },
        { fr: "S'il vous plaît, répétez.", en: "Please repeat. — to a stranger", audio: "n1u02-g04" }
      ]
    },
    dialogue: {
      title: "At the market",
      lines: [
        { speaker: "A", fr: "Salut ! Tu vas bien ?", en: "Hi! Are you well?", audio: "n1u02-d01" },
        { speaker: "B", fr: "Salut ! Oui, ça va bien, merci. Et toi ?", en: "Hi! Yes, I'm well, thanks. And you?", audio: "n1u02-d02" },
        { speaker: "A", fr: "Très bien ! Tu parles français ?", en: "Very well! Do you speak French?", audio: "n1u02-d03" },
        { speaker: "B", fr: "Oui, un peu. Et toi ?", en: "Yes, a little. And you?", audio: "n1u02-d04" },
        { speaker: "A", fr: "Moi aussi, un peu.", en: "Me too, a little.", audio: "n1u02-d05" },
        { speaker: "B", fr: "Bonjour, monsieur !", en: "Hello, sir!", audio: "n1u02-d06" },
        { speaker: "A", fr: "Bonjour ! Comment allez-vous ?", en: "Hello! How are you?", audio: "n1u02-d07" },
        { speaker: "B", fr: "Très bien, merci. S'il vous plaît, parlez lentement.", en: "Very well, thanks. Please speak slowly.", audio: "n1u02-d08" },
        { speaker: "A", fr: "Oui, pardon. Vous parlez français ?", en: "Yes, sorry. Do you speak French?", audio: "n1u02-d09" },
        { speaker: "B", fr: "Un peu. Merci, au revoir !", en: "A little. Thanks, goodbye!", audio: "n1u02-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "You're talking to a shopkeeper. 'Do you speak French?' is...", choices: ["Vous parlez français ?", "Tu parles français ?", "Vous parle français ?", "Tu parlez français ?"], answer: 0, explain: "Strangers get 'vous' — and the verb ends in -ez: 'vous parlez'.", audio: "n1u02-q01" },
      { type: "choice", q: "Your friend didn't hear you. You say...", choices: ["S'il te plaît, répète.", "S'il vous plaît, répétez.", "S'il te plaît, répétez.", "S'il vous plaît, répète."], answer: 0, explain: "Friend = tu: 's'il te plaît' pairs with 'répète'.", audio: "n1u02-q02" },
      { type: "choice", q: "'See you later today' is...", choices: ["À tout à l'heure", "À bientôt", "À demain", "Au revoir"], answer: 0, explain: "'À tout à l'heure' = later today; à bientôt = sometime soon; à demain = tomorrow.", audio: "n1u02-q03" },
      { type: "choice", q: "Which pair shows the tu / vous verb difference?", choices: ["tu parles / vous parlez", "tu parle / vous parle", "tu parlez / vous parles", "tu parler / vous parler"], answer: 0, explain: "'tu' takes -s, 'vous' takes -ez.", audio: "n1u02-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u02-q05", transcript: "Tu parles français ?", choices: ["Tu parles français ?", "Vous parlez français ?", "Tu parles anglais ?", "Vous parlez français."], answer: 0, explain: "Hear [ty] vs [vu], and the -s vs -ez verb ending." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u02-q06", transcript: "S'il vous plaît, répétez.", choices: ["S'il vous plaît, répétez.", "S'il te plaît, répète.", "S'il vous plaît, répète.", "S'il vous plaît, parlez."], answer: 0, explain: "Formal 'vous' pairs with 'répétez' [ʁepete]." }
    ]
  },
  {
    id: "n1u03", level: 1,
    title: "Introducing Yourself", titleFr: "Je me présente",
    icon: "🪪", color: "#2E9E5B",
    objectives: [
      "Introduce yourself: name, origin, nationality and profession",
      "Conjugate 's'appeler' and 'être' for je / tu / il / vous",
      "Ask and answer 'where are you from?' in both registers"
    ],
    imagePrompt: "flat vector illustration of a smiling person holding a name badge reading je m'appelle, small Canadian and French flags beside",
    vocab: [
      { fr: "Canadien / Canadienne", en: "Canadian", tip: "masculine / feminine", audio: "n1u03-v01" },
      { fr: "Français / Française", en: "French", tip: "masculine / feminine", audio: "n1u03-v02" },
      { fr: "Chinois / Chinoise", en: "Chinese", tip: "masculine / feminine", audio: "n1u03-v03" },
      { fr: "Américain / Américaine", en: "American", tip: "masculine / feminine", audio: "n1u03-v04" },
      { fr: "Étudiant / Étudiante", en: "Student", tip: "the final t sounds only in the feminine: [etydjɑ̃t]", audio: "n1u03-v05" },
      { fr: "Professeur", en: "Teacher, professor", audio: "n1u03-v06" },
      { fr: "Ingénieur", en: "Engineer", tip: "nasal 'in' [ɛ̃]", audio: "n1u03-v07" },
      { fr: "Médecin", en: "Doctor", tip: "nasal 'in'; the d is silent", audio: "n1u03-v08" },
      { fr: "Comment", en: "How", tip: "nasal 'en': [kɔmɑ̃]", audio: "n1u03-v09" },
      { fr: "Où", en: "Where", tip: "the accent marks it off from 'ou' (or)", audio: "n1u03-v10" },
      { fr: "Quel / Quelle", en: "Which, what", tip: "quel + masculine, quelle + feminine", audio: "n1u03-v11" },
      { fr: "Content / Contente", en: "Happy, pleased", tip: "masculine / feminine", audio: "n1u03-v12" }
    ],
    phrases: [
      { fr: "Je m'appelle Marie.", en: "My name is Marie.", audio: "n1u03-p01" },
      { fr: "Je suis canadienne.", en: "I'm Canadian.", tip: "a woman speaking — note the -ne ending", audio: "n1u03-p02" },
      { fr: "Je suis étudiant.", en: "I'm a student.", tip: "a man speaking", audio: "n1u03-p03" },
      { fr: "Tu es d'où ?", en: "Where are you from? (informal)", audio: "n1u03-p04" },
      { fr: "Vous êtes d'où ?", en: "Where are you from? (formal)", audio: "n1u03-p05" },
      { fr: "Je suis de Toronto.", en: "I'm from Toronto.", audio: "n1u03-p06" },
      { fr: "Enchanté / Enchantée !", en: "Nice to meet you!", audio: "n1u03-p07" }
    ],
    pronunciation: {
      title: "É vs è, and verbs that lose their endings",
      points: [
        "'é' [e] is bright and closed ('café', 'répétez'); 'è' [ɛ] is open ('m'appelle' [mapɛl], 'mère'). Mixing them is the number-one beginner giveaway.",
        "Verb endings in -ent are silent: 'ils parlent' sounds exactly like 'il parle' [paʁl]. Only the pronoun tells you singular or plural.",
        "'Je' shrinks before a vowel: je + appelle = 'j'appelle' [ʒapɛl]. Never say 'je appelle'."
      ]
    },
    grammar: {
      title: "S'appeler and être (je / tu / il / vous)",
      explanation: "Two verbs introduce you to the world. 'S'appeler' (to call oneself) gives your name: je m'appelle, tu t'appelles, il s'appelle, vous vous appelez. 'Être' (to be) gives everything else: je suis, tu es, il est, vous êtes. Notice the pattern: the little word before the verb (me / te / se / vous) always matches who you are talking about.",
      examples: [
        { fr: "Je m'appelle Aïcha et je suis ingénieure.", en: "My name is Aïcha and I'm an engineer.", audio: "n1u03-g01" },
        { fr: "Tu t'appelles comment ? Tu es d'où ?", en: "What's your name? Where are you from? (informal)", audio: "n1u03-g02" },
        { fr: "Il s'appelle Marc. Il est professeur.", en: "His name is Marc. He's a teacher.", audio: "n1u03-g03" },
        { fr: "Vous vous appelez comment ? Vous êtes canadienne ?", en: "What's your name? Are you Canadian? (formal)", audio: "n1u03-g04" }
      ]
    },
    dialogue: {
      title: "Nice to meet you",
      lines: [
        { speaker: "A", fr: "Bonjour ! Je m'appelle Sophie. Et vous ?", en: "Hello! My name is Sophie. And you?", audio: "n1u03-d01" },
        { speaker: "B", fr: "Bonjour, Sophie ! Je m'appelle Li. Enchanté !", en: "Hello, Sophie! My name is Li. Nice to meet you!", audio: "n1u03-d02" },
        { speaker: "A", fr: "Enchantée ! Vous êtes d'où, Li ?", en: "Nice to meet you! Where are you from, Li?", audio: "n1u03-d03" },
        { speaker: "B", fr: "Je suis de Vancouver. Je suis canadien. Et vous ?", en: "I'm from Vancouver. I'm Canadian. And you?", audio: "n1u03-d04" },
        { speaker: "A", fr: "Je suis de Montréal. Je suis canadienne aussi.", en: "I'm from Montréal. I'm Canadian too.", audio: "n1u03-d05" },
        { speaker: "B", fr: "Vous êtes étudiante ?", en: "Are you a student?", audio: "n1u03-d06" },
        { speaker: "A", fr: "Oui, je suis étudiante. Et vous, vous êtes ingénieur ?", en: "Yes, I'm a student. And you, are you an engineer?", audio: "n1u03-d07" },
        { speaker: "B", fr: "Non, je suis médecin.", en: "No, I'm a doctor.", audio: "n1u03-d08" },
        { speaker: "A", fr: "Enchantée ! À bientôt !", en: "Nice to meet you! See you soon!", audio: "n1u03-d09" },
        { speaker: "B", fr: "À bientôt, Sophie !", en: "See you soon, Sophie!", audio: "n1u03-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'My name is...' is...", choices: ["Je m'appelle", "Je suis appelle", "J'appelle me", "Moi appelle"], answer: 0, explain: "'S'appeler' means 'to call oneself': je m'appelle.", audio: "n1u03-q01" },
      { type: "choice", q: "To a stranger: 'Where are you from?'", choices: ["Vous êtes d'où ?", "Tu es d'où ?", "Vous es d'où ?", "Tu êtes d'où ?"], answer: 0, explain: "Stranger = vous, so the verb is 'vous êtes'.", audio: "n1u03-q02" },
      { type: "choice", q: "'He is French' is...", choices: ["Il est français.", "Il es français.", "Il est française.", "Ils est français."], answer: 0, explain: "'il est' + masculine 'français'.", audio: "n1u03-q03" },
      { type: "choice", q: "The feminine of 'canadien' is...", choices: ["canadienne", "canadiene", "canadien", "canadiennes"], answer: 0, explain: "-ien becomes -ienne: canadien → canadienne.", audio: "n1u03-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u03-q05", transcript: "Je suis étudiante.", choices: ["Je suis étudiante.", "Je suis étudiant.", "Je suis contente.", "Tu es étudiante."], answer: 0, explain: "The feminine -e makes the t sound: [etydjɑ̃t] vs [etydjɑ̃]." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u03-q06", transcript: "Vous vous appelez comment ?", choices: ["Vous vous appelez comment ?", "Vous appelez comment ?", "Tu t'appelles comment ?", "Vous vous appelez pourquoi ?"], answer: 0, explain: "Catch the doubled 'vous vous' [vuvu] and 'comment' vs 'pourquoi'." }
    ]
  },
  {
    id: "n1u04", level: 1,
    title: "Numbers 0–100", titleFr: "Les nombres",
    icon: "🔢", color: "#F4A261",
    objectives: [
      "Say and understand any number from 0 to 100",
      "Give a phone number the French way (read in digit pairs)",
      "Build 70–99 with the soixante / quatre-vingt system and dodge the classic traps"
    ],
    imagePrompt: "flat vector illustration of a French market stall with hanging price tags showing 70 80 90 euros and a pile of coins",
    vocab: [
      { fr: "zéro", en: "zero", audio: "n1u04-v01" },
      { fr: "un", en: "one", tip: "also 'a' (masculine): un café", audio: "n1u04-v02" },
      { fr: "deux", en: "two", tip: "the x is silent: [dø]", audio: "n1u04-v03" },
      { fr: "trois", en: "three", audio: "n1u04-v04" },
      { fr: "quatre", en: "four", tip: "the final e is whispered", audio: "n1u04-v05" },
      { fr: "cinq", en: "five", tip: "the q is pronounced: [sɛ̃k]", audio: "n1u04-v06" },
      { fr: "six", en: "six", tip: "the x is silent: [sis]", audio: "n1u04-v07" },
      { fr: "sept", en: "seven", tip: "the p is silent: [sɛt]", audio: "n1u04-v08" },
      { fr: "huit", en: "eight", tip: "h is silent: [ɥit]", audio: "n1u04-v09" },
      { fr: "neuf", en: "nine", tip: "the f is pronounced: [nœf]", audio: "n1u04-v10" },
      { fr: "dix", en: "ten", tip: "the x is silent: [dis] — but [diz] in 'dix-sept'!", audio: "n1u04-v11" },
      { fr: "cent", en: "a hundred", tip: "the t is silent: [sɑ̃]", audio: "n1u04-v12" }
    ],
    phrases: [
      { fr: "Quel est le numéro de téléphone ?", en: "What is the phone number?", tip: "numéro de téléphone = phone number", audio: "n1u04-p01" },
      { fr: "C'est le zéro six, douze, trente-quatre.", en: "It's 06 12 34.", tip: "phone numbers are read in pairs", audio: "n1u04-p02" },
      { fr: "Ça fait combien ?", en: "How much is that?", tip: "fixed phrase", audio: "n1u04-p03" },
      { fr: "Ça fait quinze euros.", en: "That's fifteen euros.", audio: "n1u04-p04" },
      { fr: "Et avec ceci ?", en: "And with this?", tip: "ceci = this one", audio: "n1u04-p05" },
      { fr: "C'est tout.", en: "That's all.", audio: "n1u04-p06" },
      { fr: "Merci, au revoir !", en: "Thanks, goodbye!", audio: "n1u04-p07" }
    ],
    pronunciation: {
      title: "Numbers are a pronunciation workout",
      points: [
        "'x' is silent in 'deux' [dø], 'six' [sis], 'dix' [dis] — but it buzzes as [z] in 'dix-sept' [dizsɛt] and 'dix-neuf'.",
        "Nasal vowels carry the tens: 'trente' [tʁɑ̃t], 'cent' [sɑ̃], 'vingt' [vɛ̃]. Keep the air in your nose.",
        "Stress the LAST syllable: soixante-DIX, quatre-VINGTS. Flat English stress makes numbers hard to catch."
      ]
    },
    grammar: {
      title: "Building numbers: the rules (and the weird 70–99)",
      explanation: "After 16, French builds numbers with logic — except 70 to 99, which are famously weird. 17–19 are 'ten-seven' (dix-sept, dix-huit, dix-neuf). 20–69 combine tens and ones (trente-quatre), with 'et' only in 21, 31, 41, 51, 61 (vingt et un). Then: 70 = sixty-ten (soixante-dix), 80 = four-twenties (quatre-vingts), 90 = four-twenty-ten (quatre-vingt-dix). Examiners love testing these — drill them until they're automatic.",
      examples: [
        { fr: "soixante-dix", en: "seventy — literally 'sixty-ten'", audio: "n1u04-g01" },
        { fr: "quatre-vingts", en: "eighty — 'four-twenties'; the s drops in 'quatre-vingt-deux'", audio: "n1u04-g02" },
        { fr: "quatre-vingt-dix", en: "ninety — 'four-twenty-ten'", audio: "n1u04-g03" },
        { fr: "vingt et un / trente et une", en: "21 / 31 — 'et' only for 21, 31, 41, 51, 61, never for 71, 81, 91", audio: "n1u04-g04" }
      ]
    },
    dialogue: {
      title: "At the market (vendeur / cliente)",
      lines: [
        { speaker: "A", fr: "Bonjour !", en: "Hello!", audio: "n1u04-d01" },
        { speaker: "B", fr: "Bonjour ! Ça fait combien ?", en: "Hello! How much is that?", audio: "n1u04-d02" },
        { speaker: "A", fr: "Ça fait douze euros.", en: "That's twelve euros.", audio: "n1u04-d03" },
        { speaker: "B", fr: "Douze euros ?", en: "Twelve euros?", audio: "n1u04-d04" },
        { speaker: "A", fr: "Oui, douze. Et avec ceci ?", en: "Yes, twelve. And with this?", audio: "n1u04-d05" },
        { speaker: "B", fr: "Non, c'est tout, merci.", en: "No, that's all, thanks.", audio: "n1u04-d06" },
        { speaker: "A", fr: "Très bien. Le numéro de téléphone, s'il vous plaît ?", en: "Very good. The phone number, please?", audio: "n1u04-d07" },
        { speaker: "B", fr: "C'est le zéro six, vingt-deux, quarante-cinq, soixante-dix.", en: "It's 06 22 45 70.", audio: "n1u04-d08" },
        { speaker: "A", fr: "Zéro six, vingt-deux, quarante-cinq, soixante-dix. Merci !", en: "06 22 45 70. Thanks!", audio: "n1u04-d09" },
        { speaker: "B", fr: "Merci, au revoir !", en: "Thanks, goodbye!", audio: "n1u04-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "70 in French is...", choices: ["soixante-dix", "septante", "soixante et dix", "soixante-dixs"], answer: 0, explain: "60 + 10 = soixante-dix. 'Septante' is Belgium and Switzerland — not France or Canada.", audio: "n1u04-q01" },
      { type: "choice", q: "80 in French is...", choices: ["quatre-vingts", "quatre-vingt", "octante", "quatre vingts"], answer: 0, explain: "4 × 20 = quatre-vingts (with an s). 'Octante' is Swiss.", audio: "n1u04-q02" },
      { type: "choice", q: "21 in French is...", choices: ["vingt et un", "vingt-un", "vingt et une", "vingt un"], answer: 0, explain: "'et' appears only in 21, 31, 41, 51, 61 — and 'un' is masculine here.", audio: "n1u04-q03" },
      { type: "choice", q: "How do you read 06 12 34?", choices: ["zéro six, douze, trente-quatre", "six, douze, trente-quatre", "zéro six, douze, trente et quatre", "zéro six, douze, trente quatre"], answer: 0, explain: "Keep the zéro, read in pairs, no 'et' in 34.", audio: "n1u04-q04" },
      { type: "listening", q: "Listen and choose what you hear. (Number dictation)", audio: "n1u04-q05", transcript: "soixante-quinze", choices: ["soixante-quinze", "soixante-cinq", "soixante et quinze", "septante-cinq"], answer: 0, explain: "75 = 60 + 15. No 'et' — and never 'septante' in Canada." },
      { type: "listening", q: "Listen and choose what you hear. (Number dictation)", audio: "n1u04-q06", transcript: "quatre-vingt-treize", choices: ["quatre-vingt-treize", "quatre-vingt-trois", "quatre-vingt-seize", "quatre-vingt et treize"], answer: 0, explain: "93 = 4×20 + 13. The s of 'quatre-vingts' vanishes before another number." }
    ]
  },
  {
    id: "n1u05", level: 1,
    title: "Family", titleFr: "La famille",
    icon: "👨‍👩‍👧", color: "#9B5DE5",
    objectives: [
      "Name 12 family members with the correct gender",
      "Use mon / ma / mes correctly — including the 'mon amie' vowel trap",
      "Introduce your family to someone"
    ],
    imagePrompt: "flat vector illustration of a family tree with labeled cartoon parents grandparents brother and sister",
    vocab: [
      { fr: "Le père", en: "father", tip: "open è: [pɛʁ]", audio: "n1u05-v01" },
      { fr: "La mère", en: "mother", audio: "n1u05-v02" },
      { fr: "Le frère", en: "brother", audio: "n1u05-v03" },
      { fr: "La sœur", en: "sister", tip: "the 'œu' sound [œ]", audio: "n1u05-v04" },
      { fr: "Le fils", en: "son", tip: "the s IS pronounced: [fis]", audio: "n1u05-v05" },
      { fr: "La fille", en: "daughter", tip: "'ille' = [ij]: [fij]", audio: "n1u05-v06" },
      { fr: "Le mari", en: "husband", audio: "n1u05-v07" },
      { fr: "La femme", en: "wife", tip: "the e sounds like 'a': [fam]", audio: "n1u05-v08" },
      { fr: "Les parents", en: "parents", audio: "n1u05-v09" },
      { fr: "Le grand-père", en: "grandfather", audio: "n1u05-v10" },
      { fr: "La grand-mère", en: "grandmother", audio: "n1u05-v11" },
      { fr: "Un ami / Une amie", en: "friend (m. / f.)", tip: "feminine takes 'mon': mon amie (see grammar!)", audio: "n1u05-v12" }
    ],
    phrases: [
      { fr: "Je vous présente ma famille.", en: "Let me introduce my family to you.", tip: "présenter = to introduce", audio: "n1u05-p01" },
      { fr: "Voici mon père.", en: "Here is my father.", audio: "n1u05-p02" },
      { fr: "Voici ma mère.", en: "Here is my mother.", audio: "n1u05-p03" },
      { fr: "Mon frère s'appelle Karim.", en: "My brother's name is Karim.", audio: "n1u05-p04" },
      { fr: "Ma sœur est professeure.", en: "My sister is a teacher.", tip: "feminine form: professeure", audio: "n1u05-p05" },
      { fr: "C'est mon fils.", en: "This is my son.", audio: "n1u05-p06" },
      { fr: "C'est ma fille.", en: "This is my daughter.", audio: "n1u05-p07" }
    ],
    pronunciation: {
      title: "Family words are full of è and silent letters",
      points: [
        "'père, mère, frère' all use the open è [ɛ] — don't say 'may-re', say [mɛʁ].",
        "'fille' is [fij] — the 'ille' after a vowel makes a y-glide, like the y in 'yes'.",
        "'fils' keeps its s: [fis]. And 'femme' cheats — the e sounds like 'a': [fam]."
      ]
    },
    grammar: {
      title: "Mon / ma / mes — possessives",
      explanation: "Possessives agree with the thing owned, not the owner: 'mon père' (my father — masculine), 'ma mère' (my mother — feminine), 'mes parents' (my parents — plural). One famous exception: a feminine word starting with a vowel or silent h takes 'mon' for smooth sound — 'mon amie' (my female friend), never 'ma amie'.",
      examples: [
        { fr: "mon père", en: "my father — masculine → mon", audio: "n1u05-g01" },
        { fr: "ma mère", en: "my mother — feminine → ma", audio: "n1u05-g02" },
        { fr: "mes frères", en: "my brothers — plural → mes", audio: "n1u05-g03" },
        { fr: "mon amie", en: "my (female) friend — feminine word starting with a vowel → mon", audio: "n1u05-g04" }
      ]
    },
    dialogue: {
      title: "Meet the family",
      lines: [
        { speaker: "A", fr: "Salut ! Tu es d'où ?", en: "Hi! Where are you from?", audio: "n1u05-d01" },
        { speaker: "B", fr: "Je suis de Québec. Voici ma famille !", en: "I'm from Québec. Here is my family!", audio: "n1u05-d02" },
        { speaker: "B", fr: "Voici mon père. Il s'appelle Jean.", en: "This is my father. His name is Jean.", audio: "n1u05-d03" },
        { speaker: "A", fr: "Bonjour, monsieur !", en: "Hello, sir!", audio: "n1u05-d04" },
        { speaker: "B", fr: "Voici ma mère, Marie.", en: "This is my mother, Marie.", audio: "n1u05-d05" },
        { speaker: "A", fr: "Bonjour, madame !", en: "Hello, madam!", audio: "n1u05-d06" },
        { speaker: "B", fr: "Mon frère s'appelle Luc.", en: "My brother's name is Luc.", audio: "n1u05-d07" },
        { speaker: "A", fr: "Salut, Luc !", en: "Hi, Luc!", audio: "n1u05-d08" },
        { speaker: "B", fr: "Salut ! Voici mon amie Sara.", en: "Hi! This is my friend Sara.", audio: "n1u05-d09" },
        { speaker: "A", fr: "Enchantée, Sara !", en: "Nice to meet you, Sara!", audio: "n1u05-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'My mother' is...", choices: ["ma mère", "mon mère", "mes mère", "ma mères"], answer: 0, explain: "'mère' is feminine → ma.", audio: "n1u05-q01" },
      { type: "choice", q: "'My brothers' is...", choices: ["mes frères", "mon frères", "mes frère", "ma frères"], answer: 0, explain: "Plural → mes, and 'frères' keeps its s.", audio: "n1u05-q02" },
      { type: "choice", q: "'My (female) friend' is...", choices: ["mon amie", "ma amie", "mes amie", "mon ami"], answer: 0, explain: "A feminine word starting with a vowel takes 'mon': mon amie. ('Mon ami' = a male friend.)", audio: "n1u05-q03" },
      { type: "choice", q: "Which word is feminine?", choices: ["la sœur", "le frère", "le père", "le fils"], answer: 0, explain: "'la sœur' — the others are masculine (le).", audio: "n1u05-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u05-q05", transcript: "Voici ma sœur.", choices: ["Voici ma sœur.", "Voici mes sœurs.", "Voici mon sœur.", "Voici ma frère."], answer: 0, explain: "Hear ma [ma] vs mes [me] vs mon [mɔ̃]." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u05-q06", transcript: "Mon frère s'appelle Luc.", choices: ["Mon frère s'appelle Luc.", "Ma frère s'appelle Luc.", "Mon frère s'appelle Lucas.", "Mon frère s'appelle Marc."], answer: 0, explain: "Luc [lyk] vs Lucas [lyka] vs Marc [maʁk] — catch the ending." }
    ]
  },
  {
    id: "n1u06", level: 1,
    title: "Food and Drink", titleFr: "La nourriture",
    icon: "🥐", color: "#00BBF9",
    objectives: [
      "Order quantities with du / de la / de l' / des automatically",
      "Say what you like and don't like (j'aime / je n'aime pas)",
      "Use 'de' — not 'du' — after a negation"
    ],
    imagePrompt: "flat vector illustration of a French breakfast table with labeled baguette cheese coffee cup and croissant",
    vocab: [
      { fr: "Le pain", en: "bread", tip: "nasal 'ain' [ɛ̃]", audio: "n1u06-v01" },
      { fr: "Le fromage", en: "cheese", tip: "g + e = [ʒ]: [fʁɔmaʒ]", audio: "n1u06-v02" },
      { fr: "Le lait", en: "milk", tip: "'ai' = [ɛ]: [lɛ]", audio: "n1u06-v03" },
      { fr: "Le café", en: "coffee", tip: "bright é [e]", audio: "n1u06-v04" },
      { fr: "Le thé", en: "tea", audio: "n1u06-v05" },
      { fr: "L'eau", en: "water", tip: "'eau' = [o]: [lo]", audio: "n1u06-v06" },
      { fr: "La viande", en: "meat", tip: "'ian' = [jɑ̃]", audio: "n1u06-v07" },
      { fr: "Le poisson", en: "fish", tip: "'oi' = [wa]: [pwasɔ̃]", audio: "n1u06-v08" },
      { fr: "Le gâteau", en: "cake", tip: "'eau' = [o]; the final u is silent", audio: "n1u06-v09" },
      { fr: "Manger", en: "to eat", tip: "g + e = [ʒ]: [mɑ̃ʒe]", audio: "n1u06-v10" },
      { fr: "Boire", en: "to drink", tip: "'oi' = [wa]: [bwaʁ]", audio: "n1u06-v11" },
      { fr: "Aimer", en: "to like, to love", tip: "'ai' = [ɛ]: [eme]", audio: "n1u06-v12" }
    ],
    phrases: [
      { fr: "J'aime le fromage.", en: "I like cheese.", audio: "n1u06-p01" },
      { fr: "Je n'aime pas le poisson.", en: "I don't like fish.", audio: "n1u06-p02" },
      { fr: "Du pain, s'il vous plaît.", en: "Some bread, please.", audio: "n1u06-p03" },
      { fr: "De l'eau, s'il vous plaît.", en: "Some water, please.", audio: "n1u06-p04" },
      { fr: "Je n'aime pas la viande.", en: "I don't like meat.", audio: "n1u06-p05" },
      { fr: "J'aime beaucoup le gâteau.", en: "I like cake a lot.", tip: "beaucoup = a lot", audio: "n1u06-p06" },
      { fr: "C'est délicieux !", en: "It's delicious!", tip: "délicieux = delicious", audio: "n1u06-p07" }
    ],
    pronunciation: {
      title: "The great vowel combos: eau, ai, oi",
      points: [
        "'eau' is always [o]: l'eau [lo], gâteau [gato]. Three letters, one sound.",
        "'ai' is [ɛ]: lait [lɛ], aimer [eme], viande [vjɑ̃d]. Don't say 'lay'.",
        "'oi' is [wa]: poisson [pwasɔ̃], boire [bwaʁ]. And g + e/i = [ʒ]: fromage [fʁɔmaʒ], manger [mɑ̃ʒe]."
      ]
    },
    grammar: {
      title: "Du / de la / de l' / des — the partitive articles",
      explanation: "When you mean 'some' of something, French needs a partitive article: 'du' for masculine (du pain), 'de la' for feminine (de la viande), 'de l'' before a vowel (de l'eau), 'des' for plural. But after a negation, they all collapse to 'de': 'Je mange du pain' becomes 'Je ne mange pas de pain'. For general likes and dislikes, use le/la instead: 'J'aime le fromage' (cheese in general).",
      examples: [
        { fr: "Je mange du pain.", en: "I eat (some) bread — masculine → du", audio: "n1u06-g01" },
        { fr: "Je bois de l'eau.", en: "I drink (some) water — before a vowel → de l'", audio: "n1u06-g02" },
        { fr: "Je mange de la viande.", en: "I eat (some) meat — feminine → de la", audio: "n1u06-g03" },
        { fr: "Je ne bois pas de lait.", en: "I don't drink milk — after negation → de, no article!", audio: "n1u06-g04" }
      ]
    },
    dialogue: {
      title: "Lunch",
      lines: [
        { speaker: "A", fr: "Tu aimes le fromage ?", en: "Do you like cheese?", audio: "n1u06-d01" },
        { speaker: "B", fr: "Oui, j'aime beaucoup le fromage ! Et toi ?", en: "Yes, I like cheese a lot! And you?", audio: "n1u06-d02" },
        { speaker: "A", fr: "Moi aussi ! Tu bois du café ?", en: "Me too! Do you drink coffee?", audio: "n1u06-d03" },
        { speaker: "B", fr: "Non, je ne bois pas de café. Je bois du thé.", en: "No, I don't drink coffee. I drink tea.", audio: "n1u06-d04" },
        { speaker: "A", fr: "Moi, je bois du lait.", en: "I drink milk.", audio: "n1u06-d05" },
        { speaker: "B", fr: "Tu manges de la viande ?", en: "Do you eat meat?", audio: "n1u06-d06" },
        { speaker: "A", fr: "Non, je ne mange pas de viande. Je mange du poisson.", en: "No, I don't eat meat. I eat fish.", audio: "n1u06-d07" },
        { speaker: "B", fr: "Et le gâteau ? Tu aimes le gâteau ?", en: "And cake? Do you like cake?", audio: "n1u06-d08" },
        { speaker: "A", fr: "Oui ! Le gâteau, c'est délicieux !", en: "Yes! Cake is delicious!", audio: "n1u06-d09" },
        { speaker: "B", fr: "Alors, du gâteau et du thé, s'il vous plaît !", en: "So, some cake and some tea, please!", audio: "n1u06-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'Some bread' (masculine) is...", choices: ["du pain", "de la pain", "des pain", "le pain"], answer: 0, explain: "Masculine → du.", audio: "n1u06-q01" },
      { type: "choice", q: "'Some water' is...", choices: ["de l'eau", "de la eau", "du eau", "des eaux"], answer: 0, explain: "Before a vowel → de l'.", audio: "n1u06-q02" },
      { type: "choice", q: "'I don't drink milk' is...", choices: ["Je ne bois pas de lait.", "Je ne bois pas du lait.", "Je ne bois pas de le lait.", "Je ne bois pas lait."], answer: 0, explain: "After negation the article collapses to 'de'.", audio: "n1u06-q03" },
      { type: "choice", q: "'I like fish' (in general) is...", choices: ["J'aime le poisson.", "J'aime du poisson.", "Je aime le poisson.", "J'aime les poisson."], answer: 0, explain: "General likes use le/la; and 'je' becomes j' before a vowel.", audio: "n1u06-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u06-q05", transcript: "Je ne mange pas de viande.", choices: ["Je ne mange pas de viande.", "Je ne mange pas de la viande.", "Je mange pas de viande.", "Je ne mange pas des viandes."], answer: 0, explain: "In writing keep the 'ne'; after negation it's 'de viande'." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u06-q06", transcript: "Du thé, s'il vous plaît.", choices: ["Du thé, s'il vous plaît.", "De thé, s'il vous plaît.", "Du thé, s'il te plaît.", "Des thés, s'il vous plaît."], answer: 0, explain: "Hear [dy] vs [də], and vous [vu] vs te [tə]." }
    ]
  },
  {
    id: "n1u07", level: 1,
    title: "At the Café", titleFr: "Au café",
    icon: "☕", color: "#F15BB5",
    objectives: [
      "Order food and drink politely with 'je voudrais'",
      "Ask for the bill and pay, by cash or card",
      "Use the je / tu / vous forms of 'vouloir' and 'pouvoir'"
    ],
    imagePrompt: "flat vector illustration of a Parisian café counter with a waiter, a menu card labeled la carte, a coffee cup and a bill labeled l'addition",
    vocab: [
      { fr: "La carte", en: "menu", tip: "not 'menu' — in France that's the fixed-price meal!", audio: "n1u07-v01" },
      { fr: "L'addition", en: "bill, check", audio: "n1u07-v02" },
      { fr: "Le serveur", en: "waiter", tip: "'eu' = [œ]: [sɛʁvœʁ]", audio: "n1u07-v03" },
      { fr: "La serveuse", en: "waitress", audio: "n1u07-v04" },
      { fr: "La tasse", en: "cup", tip: "the final e is whispered", audio: "n1u07-v05" },
      { fr: "Le verre", en: "glass", tip: "double r, deep in the throat", audio: "n1u07-v06" },
      { fr: "La table", en: "table", audio: "n1u07-v07" },
      { fr: "La terrasse", en: "terrace", tip: "sitting outside", audio: "n1u07-v08" },
      { fr: "Vouloir", en: "to want", tip: "nasal vowel + [waʁ]", audio: "n1u07-v09" },
      { fr: "Pouvoir", en: "to be able to", tip: "[puvwaʁ]", audio: "n1u07-v10" },
      { fr: "Payer", en: "to pay", tip: "'ay' = [ej]: [peje]", audio: "n1u07-v11" },
      { fr: "Cher / Chère", en: "expensive", tip: "masculine / feminine", audio: "n1u07-v12" }
    ],
    phrases: [
      { fr: "Une table pour deux, s'il vous plaît.", en: "A table for two, please.", tip: "pour = for", audio: "n1u07-p01" },
      { fr: "La carte, s'il vous plaît.", en: "The menu, please.", audio: "n1u07-p02" },
      { fr: "Je voudrais un café.", en: "I would like a coffee.", audio: "n1u07-p03" },
      { fr: "Je voudrais un thé et un gâteau.", en: "I would like a tea and a cake.", audio: "n1u07-p04" },
      { fr: "L'addition, s'il vous plaît.", en: "The bill, please.", audio: "n1u07-p05" },
      { fr: "Je peux payer par carte ?", en: "Can I pay by card?", tip: "par carte = by bank card", audio: "n1u07-p06" },
      { fr: "C'est combien ?", en: "How much is it?", audio: "n1u07-p07" }
    ],
    pronunciation: {
      title: "Polite sounds: eu, and softening your requests",
      points: [
        "'eu' in 'serveur' is [œ] — relaxed, like 'uh': [sɛʁvœʁ]. Don't say 'serve-oor'.",
        "'je voudrais' [ʒə vudʁɛ] ends open and soft — that gentle [ɛ] ending is exactly what makes it polite.",
        "'ch' is always [ʃ]: cher [ʃɛʁ]. And 'th' is just [t]: thé [te] — never the English 'th'."
      ]
    },
    grammar: {
      title: "Vouloir / pouvoir + the magic 'je voudrais'",
      explanation: "'Vouloir' (to want): je veux, tu veux, vous voulez. 'Pouvoir' (can, to be able to): je peux, tu peux, vous pouvez. At a café, never bark 'je veux' at a stranger — use the conditional 'je voudrais' (I would like), the magic politeness word. 'Tu peux' is for friends; 'vous pouvez' opens every polite request to strangers.",
      examples: [
        { fr: "Je veux un café.", en: "I want a coffee — direct, fine with friends", audio: "n1u07-g01" },
        { fr: "Je voudrais un café, s'il vous plaît.", en: "I would like a coffee, please — polite, always use this with strangers", audio: "n1u07-g02" },
        { fr: "Tu peux répéter ?", en: "Can you repeat? — informal", audio: "n1u07-g03" },
        { fr: "Vous pouvez parler lentement ?", en: "Can you speak slowly? — formal", audio: "n1u07-g04" }
      ]
    },
    dialogue: {
      title: "At the café (serveur / client)",
      lines: [
        { speaker: "A", fr: "Bonjour ! Une table pour deux ?", en: "Hello! A table for two?", audio: "n1u07-d01" },
        { speaker: "B", fr: "Bonjour ! Oui, s'il vous plaît.", en: "Hello! Yes, please.", audio: "n1u07-d02" },
        { speaker: "A", fr: "Voici la carte.", en: "Here's the menu.", audio: "n1u07-d03" },
        { speaker: "B", fr: "Merci. Je voudrais un café et un gâteau, s'il vous plaît.", en: "Thanks. I'd like a coffee and a cake, please.", audio: "n1u07-d04" },
        { speaker: "A", fr: "Très bien. Un café et un gâteau.", en: "Very good. A coffee and a cake.", audio: "n1u07-d05" },
        { speaker: "B", fr: "C'est combien ?", en: "How much is it?", audio: "n1u07-d06" },
        { speaker: "A", fr: "Ça fait huit euros.", en: "That's eight euros.", audio: "n1u07-d07" },
        { speaker: "B", fr: "Je peux payer par carte ?", en: "Can I pay by card?", audio: "n1u07-d08" },
        { speaker: "A", fr: "Oui.", en: "Yes.", audio: "n1u07-d09" },
        { speaker: "B", fr: "Merci, au revoir !", en: "Thanks, goodbye!", audio: "n1u07-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "The polite way to order is...", choices: ["Je voudrais", "Je veux", "Je voudrai", "Vouloir je"], answer: 0, explain: "'Je voudrais' (conditional) is the polite form; 'je veux' is blunt; 'je voudrai' is the future — wrong here.", audio: "n1u07-q01" },
      { type: "choice", q: "'Can you repeat?' (to a stranger)", choices: ["Vous pouvez répéter ?", "Vous poulez répéter ?", "Vous pouvez répète ?", "Tu peux répéter ?"], answer: 0, explain: "Stranger = vous: 'vous pouvez' + the infinitive 'répéter'.", audio: "n1u07-q02" },
      { type: "choice", q: "'The bill, please' is...", choices: ["L'addition, s'il vous plaît.", "La addition, s'il vous plaît.", "L'addition, s'il te plaît.", "Addition, s'il vous plaît."], answer: 0, explain: "'addition' is feminine but starts with a vowel → l'.", audio: "n1u07-q03" },
      { type: "choice", q: "'I want' is...", choices: ["Je veux", "Je veut", "Je voulons", "J'veux"], answer: 0, explain: "'je veux' [vø]. 'J'veux' is spoken slang — never write it.", audio: "n1u07-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u07-q05", transcript: "Je voudrais la carte.", choices: ["Je voudrais la carte.", "Je veux la carte.", "Je voudrais le carte.", "Je voudrai la carte."], answer: 0, explain: "[vudʁɛ] (would like) vs [vø] (want) vs [vudʁe] (will want)." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u07-q06", transcript: "Vous pouvez payer par carte.", choices: ["Vous pouvez payer par carte.", "Tu peux payer par carte.", "Vous voulez payer par carte.", "Vous pouvez payer en carte."], answer: 0, explain: "[puve] (can) vs [vule] (want); 'par carte' = by card." }
    ]
  },
  {
    id: "n1u08", level: 1,
    title: "Days, Dates and Time", titleFr: "Le temps",
    icon: "🕰️", color: "#06D6A0",
    objectives: [
      "Say the day, the date and the time in French",
      "Use 'il est' + heures, 'et demie' and 'moins le quart'",
      "Put months after 'en' and days with no article"
    ],
    imagePrompt: "flat vector illustration of a weekly calendar from lundi to dimanche beside a wall clock showing half past eight",
    vocab: [
      { fr: "Lundi", en: "Monday", audio: "n1u08-v01" },
      { fr: "Mardi", en: "Tuesday", audio: "n1u08-v02" },
      { fr: "Mercredi", en: "Wednesday", tip: "not 'mercerdi'!", audio: "n1u08-v03" },
      { fr: "Jeudi", en: "Thursday", audio: "n1u08-v04" },
      { fr: "Vendredi", en: "Friday", audio: "n1u08-v05" },
      { fr: "Samedi", en: "Saturday", audio: "n1u08-v06" },
      { fr: "Dimanche", en: "Sunday", tip: "nasal 'an': [dimɑ̃ʃ]", audio: "n1u08-v07" },
      { fr: "Aujourd'hui", en: "today", tip: "one word: [oʒuʁdɥi]", audio: "n1u08-v08" },
      { fr: "Demain", en: "tomorrow", tip: "nasal 'ain': [dəmɛ̃]", audio: "n1u08-v09" },
      { fr: "Hier", en: "yesterday", tip: "h silent, final r silent: [jɛʁ]", audio: "n1u08-v10" },
      { fr: "La semaine", en: "week", audio: "n1u08-v11" },
      { fr: "Le mois", en: "month", tip: "'oi' = [wa]: [mwa]", audio: "n1u08-v12" }
    ],
    phrases: [
      { fr: "Quel jour sommes-nous ?", en: "What day is it?", tip: "fixed phrase — the full 'nous' form comes in Level 2", audio: "n1u08-p01" },
      { fr: "Nous sommes lundi.", en: "It's Monday.", audio: "n1u08-p02" },
      { fr: "Aujourd'hui, c'est mardi.", en: "Today is Tuesday.", audio: "n1u08-p03" },
      { fr: "Il est trois heures.", en: "It's 3 o'clock.", audio: "n1u08-p04" },
      { fr: "Il est huit heures et demie.", en: "It's 8:30.", audio: "n1u08-p05" },
      { fr: "Mon anniversaire, c'est en juillet.", en: "My birthday is in July.", tip: "anniversaire = birthday", audio: "n1u08-p06" },
      { fr: "À quelle heure ?", en: "At what time?", tip: "l'heure = the time", audio: "n1u08-p07" }
    ],
    pronunciation: {
      title: "Time words: silent h and tricky vowels",
      points: [
        "'h' is always silent: 'heure' [œʁ], 'hier' [jɛʁ], 'huit' [ɥit]. 'L'heure' flows as one sound: [lœʁ].",
        "'aujourd'hui' is [oʒuʁdɥi] — say it in two beats, 'o-zhoor-dwee': practice slowly, then speed up.",
        "'en' = [ɑ̃]: 'en juillet' [ɑ̃ ʒɥijɛ]. And 'mercredi' — every letter counts, don't swallow the middle."
      ]
    },
    grammar: {
      title: "Il est + time; days and months",
      explanation: "'Il est' + number + 'heure(s)' tells the time: 'Il est trois heures.' Halves and quarters: 'et demie' (half past), 'et quart' (quarter past), 'moins le quart' (quarter to). Days of the week take no article for a specific day: 'Nous sommes lundi' — with 'le' it would mean 'on Mondays'. Months follow 'en': 'en juillet', 'en décembre'. The months: janvier, février, mars, avril, mai, juin, juillet, août, septembre, octobre, novembre, décembre. The seasons: le printemps, l'été, l'automne, l'hiver.",
      examples: [
        { fr: "Il est neuf heures.", en: "It's 9 o'clock.", audio: "n1u08-g01" },
        { fr: "Il est deux heures et demie.", en: "It's 2:30 — 'et demie' (demie agrees with feminine 'heure')", audio: "n1u08-g02" },
        { fr: "Nous sommes en décembre.", en: "It's December — months take 'en'", audio: "n1u08-g03" },
        { fr: "Il est huit heures moins le quart.", en: "It's quarter to nine — 'moins le quart' = quarter to", audio: "n1u08-g04" }
      ]
    },
    dialogue: {
      title: "Making a plan",
      lines: [
        { speaker: "A", fr: "Salut ! Quel jour sommes-nous ?", en: "Hi! What day is it?", audio: "n1u08-d01" },
        { speaker: "B", fr: "Nous sommes mercredi.", en: "It's Wednesday.", audio: "n1u08-d02" },
        { speaker: "A", fr: "Demain, c'est jeudi. À quelle heure ?", en: "Tomorrow is Thursday. At what time?", audio: "n1u08-d03" },
        { speaker: "B", fr: "À trois heures.", en: "At 3 o'clock.", audio: "n1u08-d04" },
        { speaker: "A", fr: "Trois heures et demie, d'accord ?", en: "Half past three, OK?", audio: "n1u08-d05" },
        { speaker: "B", fr: "D'accord ! Et samedi ?", en: "OK! And Saturday?", audio: "n1u08-d06" },
        { speaker: "A", fr: "Samedi, à dix heures.", en: "Saturday, at 10 o'clock.", audio: "n1u08-d07" },
        { speaker: "B", fr: "Dix heures et demie ?", en: "Half past ten?", audio: "n1u08-d08" },
        { speaker: "A", fr: "Non, dix heures. Ça va ?", en: "No, ten o'clock. Is that OK?", audio: "n1u08-d09" },
        { speaker: "B", fr: "Oui, ça va ! À samedi !", en: "Yes, fine! See you Saturday!", audio: "n1u08-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'It's 3:30' is...", choices: ["Il est trois heures et demie.", "Il est trois heures et demi.", "Il est trois heure et demie.", "Il est trois heures demi."], answer: 0, explain: "'demie' agrees with feminine 'heure' → et demie.", audio: "n1u08-q01" },
      { type: "choice", q: "'What day is it?' is...", choices: ["Quel jour sommes-nous ?", "Quel jour sommes-je ?", "Quelle jour sommes-nous ?", "Quel jour est-nous ?"], answer: 0, explain: "Fixed phrase: 'quel' (masculine 'jour') + 'sommes-nous'.", audio: "n1u08-q02" },
      { type: "choice", q: "Wednesday is...", choices: ["mercredi", "mercedi", "mardi", "mercrdi"], answer: 0, explain: "m-e-r-c-r-e-d-i: mercredi. Don't confuse it with mardi (Tuesday).", audio: "n1u08-q03" },
      { type: "choice", q: "'In July' is...", choices: ["en juillet", "à juillet", "en le juillet", "au juillet"], answer: 0, explain: "Months take 'en', with no article.", audio: "n1u08-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u08-q05", transcript: "Il est huit heures moins le quart.", choices: ["Il est huit heures moins le quart.", "Il est huit heures et quart.", "Il est neuf heures moins le quart.", "Il est dix-huit heures moins le quart."], answer: 0, explain: "'moins le quart' = quarter to; 'et quart' = quarter past." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u08-q06", transcript: "Nous sommes vendredi.", choices: ["Nous sommes vendredi.", "Nous sommes samedi.", "Nous sommes le vendredi.", "Nous sommes en vendredi."], answer: 0, explain: "Days take no article and no 'en' — just 'nous sommes vendredi'." }
    ]
  },
  {
    id: "n1u09", level: 1,
    title: "Être and Avoir", titleFr: "Être et avoir",
    icon: "💪", color: "#EF8354",
    objectives: [
      "Conjugate 'être' and 'avoir' fully in the present tense",
      "Say your age and use 'avoir' idioms (avoir faim, avoir froid, avoir raison)",
      "Describe people: tired, sick, married, ready"
    ],
    imagePrompt: "flat vector illustration of two columns labeled être and avoir with cartoon people demonstrating je suis tu es il est and j'ai tu as il a",
    vocab: [
      { fr: "Avoir", en: "to have", audio: "n1u09-v01" },
      { fr: "Grand / Grande", en: "big, tall", tip: "masculine / feminine", audio: "n1u09-v02" },
      { fr: "Petit / Petite", en: "small", tip: "masculine / feminine", audio: "n1u09-v03" },
      { fr: "Fatigué / Fatiguée", en: "tired", tip: "bright é ending", audio: "n1u09-v04" },
      { fr: "Malade", en: "sick", audio: "n1u09-v05" },
      { fr: "Marié / Mariée", en: "married", tip: "masculine / feminine", audio: "n1u09-v06" },
      { fr: "Célibataire", en: "single", audio: "n1u09-v07" },
      { fr: "Prêt / Prête", en: "ready", tip: "final t silent in the masculine: [pʁɛ]", audio: "n1u09-v08" },
      { fr: "Chaud", en: "hot, warm", tip: "'ch' = [ʃ], d silent: [ʃo]", audio: "n1u09-v09" },
      { fr: "Froid", en: "cold", tip: "d silent: [fʁwa]", audio: "n1u09-v10" },
      { fr: "L'âge", en: "age", tip: "masculine! say 'quel âge'", audio: "n1u09-v11" },
      { fr: "L'année", en: "year", tip: "'ée' ending [e]", audio: "n1u09-v12" }
    ],
    phrases: [
      { fr: "J'ai vingt-cinq ans.", en: "I'm 25.", audio: "n1u09-p01" },
      { fr: "Quel âge as-tu ?", en: "How old are you? (informal)", audio: "n1u09-p02" },
      { fr: "Quel âge avez-vous ?", en: "How old are you? (formal)", audio: "n1u09-p03" },
      { fr: "J'ai faim.", en: "I'm hungry.", tip: "faim = hunger — French 'has' hunger", audio: "n1u09-p04" },
      { fr: "J'ai soif.", en: "I'm thirsty.", tip: "soif = thirst", audio: "n1u09-p05" },
      { fr: "Il a raison.", en: "He's right.", tip: "raison = reason — 'he has reason'", audio: "n1u09-p06" },
      { fr: "Tu as l'heure ?", en: "Do you have the time?", audio: "n1u09-p07" }
    ],
    pronunciation: {
      title: "Adjective endings you can hear",
      points: [
        "Masculine/feminine pairs often differ by ONE sounded consonant: 'grand' [gʁɑ̃] vs 'grande' [gʁɑ̃d], 'froid' [fʁwa] vs 'froide' [fʁwad]. Train your ear on the final sound.",
        "'é / ée' endings are bright [e]: fatigué [fatige], mariée [maʁje]. Keep them short and crisp.",
        "'âge' has a soft g: [aʒ], like the 's' in English 'measure'."
      ]
    },
    grammar: {
      title: "Être and avoir — full present tense",
      explanation: "The two pillars of French. Être (to be): je suis, tu es, il/elle est, nous sommes, vous êtes, ils/elles sont. Avoir (to have): j'ai, tu as, il/elle a, nous avons, vous avez, ils/elles ont. French uses 'avoir' where English uses 'to be' for age, hunger, thirst and temperature: 'J'ai 30 ans', 'J'ai faim', 'Il a froid'. Say both tables out loud every day until they're automatic.",
      examples: [
        { fr: "J'ai trente ans. Tu as quel âge ?", en: "I'm 30. How old are you?", audio: "n1u09-g01" },
        { fr: "Il est malade. Il a froid.", en: "He's sick. He's cold — feelings of temperature use 'avoir'", audio: "n1u09-g02" },
        { fr: "Vous êtes prêts ? Nous avons une table.", en: "Are you ready? We have a table.", audio: "n1u09-g03" },
        { fr: "Elle est fatiguée parce qu'elle a chaud.", en: "She's tired because she's hot.", audio: "n1u09-g04" }
      ]
    },
    dialogue: {
      title: "At the clinic (réceptionniste / patient)",
      lines: [
        { speaker: "A", fr: "Bonjour ! Vous avez quel âge ?", en: "Hello! How old are you?", audio: "n1u09-d01" },
        { speaker: "B", fr: "J'ai quarante ans.", en: "I'm 40.", audio: "n1u09-d02" },
        { speaker: "A", fr: "Vous êtes malade ?", en: "Are you sick?", audio: "n1u09-d03" },
        { speaker: "B", fr: "Oui, j'ai froid et je suis fatigué.", en: "Yes, I'm cold and I'm tired.", audio: "n1u09-d04" },
        { speaker: "A", fr: "Vous avez chaud ?", en: "Are you hot?", audio: "n1u09-d05" },
        { speaker: "B", fr: "Non, j'ai froid.", en: "No, I'm cold.", audio: "n1u09-d06" },
        { speaker: "A", fr: "D'accord. Vous êtes prêt ?", en: "OK. Are you ready?", audio: "n1u09-d07" },
        { speaker: "B", fr: "Oui, je suis prêt. Merci !", en: "Yes, I'm ready. Thanks!", audio: "n1u09-d08" },
        { speaker: "A", fr: "De rien. Au revoir !", en: "You're welcome. Goodbye!", audio: "n1u09-d09" },
        { speaker: "B", fr: "Au revoir !", en: "Goodbye!", audio: "n1u09-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'They (masc.) have' is...", choices: ["ils ont", "ils avons", "ils ontent", "ils avont"], answer: 0, explain: "'ils/elles ont' [ɔ̃] — the -ent ending is silent.", audio: "n1u09-q01" },
      { type: "choice", q: "'We are' is...", choices: ["nous sommes", "nous sont", "nous avons", "nous être"], answer: 0, explain: "'nous sommes' (we are) vs 'nous avons' (we have) — don't mix them!", audio: "n1u09-q02" },
      { type: "choice", q: "'She's cold' is...", choices: ["Elle a froid.", "Elle est froid.", "Elle a froide.", "Elle est froide."], answer: 0, explain: "Feelings of temperature use 'avoir': elle a froid.", audio: "n1u09-q03" },
      { type: "choice", q: "'I'm 25' is...", choices: ["J'ai vingt-cinq ans.", "Je suis vingt-cinq ans.", "J'ai vingt-cinq an.", "Je ai vingt-cinq ans."], answer: 0, explain: "Age uses 'avoir'; 'ans' is plural; je + ai contracts to j'ai.", audio: "n1u09-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u09-q05", transcript: "Vous avez quel âge ?", choices: ["Vous avez quel âge ?", "Tu as quel âge ?", "Vous êtes quel âge ?", "Vous avez quel âge !"], answer: 0, explain: "'âge' is masculine → 'quel'; and age takes 'avoir', never 'être'." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u09-q06", transcript: "Ils sont prêts.", choices: ["Ils sont prêts.", "Elles sont prêtes.", "Ils ont prêts.", "Ils sont malades."], answer: 0, explain: "[il sɔ̃] vs [ɛl sɔ̃] vs [il zɔ̃] — hear the pronoun and the liaison." }
    ]
  },
  {
    id: "n1u10", level: 1,
    title: "Around Town", titleFr: "En ville",
    icon: "🗺️", color: "#5E60CE",
    objectives: [
      "Ask for and understand directions (left, right, straight ahead)",
      "Use à / en / au / aux correctly with places",
      "Say where you're going with 'aller'"
    ],
    imagePrompt: "flat vector illustration of a small French town map with a bakery pharmacy train station and bank, curved direction arrows between them",
    vocab: [
      { fr: "La ville", en: "city, town", audio: "n1u10-v01" },
      { fr: "La boulangerie", en: "bakery", tip: "g + e = [ʒ]", audio: "n1u10-v02" },
      { fr: "La pharmacie", en: "pharmacy", tip: "'ph' = [f]: [faʁmasi]", audio: "n1u10-v03" },
      { fr: "Le supermarché", en: "supermarket", tip: "bright é", audio: "n1u10-v04" },
      { fr: "La gare", en: "train station", tip: "hard g [gaʁ] — not 'garde'!", audio: "n1u10-v05" },
      { fr: "L'hôpital", en: "hospital", tip: "h silent; 'ô' = [o]", audio: "n1u10-v06" },
      { fr: "La banque", en: "bank", tip: "nasal 'an': [bɑ̃k]", audio: "n1u10-v07" },
      { fr: "La rue", en: "street", tip: "throaty R + tight [y]: [ʁy]", audio: "n1u10-v08" },
      { fr: "À gauche", en: "to the left", tip: "g before a is hard [g]", audio: "n1u10-v09" },
      { fr: "À droite", en: "to the right", tip: "'oi' = [wa]: [dʁwat]", audio: "n1u10-v10" },
      { fr: "Tout droit", en: "straight ahead", tip: "final t silent: [dʁwa]", audio: "n1u10-v11" },
      { fr: "Aller", en: "to go", audio: "n1u10-v12" }
    ],
    phrases: [
      { fr: "Où est la boulangerie ?", en: "Where is the bakery?", audio: "n1u10-p01" },
      { fr: "Où sont les toilettes ?", en: "Where is the bathroom?", tip: "les toilettes = the restroom", audio: "n1u10-p02" },
      { fr: "C'est loin d'ici ?", en: "Is it far from here?", tip: "loin = far, ici = here", audio: "n1u10-p03" },
      { fr: "C'est à gauche.", en: "It's on the left.", audio: "n1u10-p04" },
      { fr: "Allez tout droit.", en: "Go straight ahead.", tip: "'allez' used as a command (formal)", audio: "n1u10-p05" },
      { fr: "Il y a une pharmacie près d'ici.", en: "There's a pharmacy near here.", tip: "près de = near", audio: "n1u10-p06" },
      { fr: "Je vais à la gare.", en: "I'm going to the station.", audio: "n1u10-p07" }
    ],
    pronunciation: {
      title: "Street French: hard g, oi, and the gare / garde trap",
      points: [
        "'gare' [gaʁ] (station) vs 'garde' [gaʁd] (guard): that final d IS pronounced in 'garde'. One consonant changes the word.",
        "'pharmacie' starts with [f]: [faʁmasi]. And 'rue' [ʁy] needs the tight French 'u' — not English 'roo'.",
        "'droite' [dʁwat] and 'tout' [tu]: keep 'oi' as [wa] and 'ou' as [u] — 'à droite' and 'tout droit' must sound different."
      ]
    },
    grammar: {
      title: "Aller + à / en / au / aux",
      explanation: "'Aller' (to go) is irregular: je vais, tu vas, il/elle va, vous allez. Destinations need the right preposition: 'à la' for feminine places (à la boulangerie), 'au' for masculine (au supermarché — à + le always contracts), 'aux' for plural (aux toilettes), and 'en' for feminine countries (en France). Cities simply take 'à': à Paris, à Toronto.",
      examples: [
        { fr: "Je vais à la boulangerie.", en: "I'm going to the bakery — à + la (feminine)", audio: "n1u10-g01" },
        { fr: "Tu vas au supermarché ?", en: "Are you going to the supermarket? — à + le = au (masculine)", audio: "n1u10-g02" },
        { fr: "Il va aux toilettes.", en: "He's going to the bathroom — à + les = aux (plural)", audio: "n1u10-g03" },
        { fr: "Vous allez en France ?", en: "Are you going to France? — en + feminine country", audio: "n1u10-g04" }
      ]
    },
    dialogue: {
      title: "Asking the way",
      lines: [
        { speaker: "A", fr: "Pardon, monsieur ! Où est la pharmacie, s'il vous plaît ?", en: "Excuse me, sir! Where is the pharmacy, please?", audio: "n1u10-d01" },
        { speaker: "B", fr: "La pharmacie ? Allez tout droit.", en: "The pharmacy? Go straight ahead.", audio: "n1u10-d02" },
        { speaker: "A", fr: "Tout droit ?", en: "Straight ahead?", audio: "n1u10-d03" },
        { speaker: "B", fr: "Oui. C'est à gauche.", en: "Yes. It's on the left.", audio: "n1u10-d04" },
        { speaker: "A", fr: "C'est loin d'ici ?", en: "Is it far from here?", audio: "n1u10-d05" },
        { speaker: "B", fr: "Non, c'est près d'ici.", en: "No, it's near here.", audio: "n1u10-d06" },
        { speaker: "A", fr: "Il y a une banque près de la pharmacie ?", en: "Is there a bank near the pharmacy?", audio: "n1u10-d07" },
        { speaker: "B", fr: "Oui, il y a une banque à droite.", en: "Yes, there's a bank on the right.", audio: "n1u10-d08" },
        { speaker: "A", fr: "Merci beaucoup, monsieur !", en: "Thank you very much, sir!", audio: "n1u10-d09" },
        { speaker: "B", fr: "De rien ! Au revoir !", en: "You're welcome! Goodbye!", audio: "n1u10-d10" }
      ]
    },
    quiz: [
      { type: "choice", q: "'To the supermarket' (masculine) is...", choices: ["au supermarché", "à le supermarché", "à la supermarché", "aux supermarché"], answer: 0, explain: "à + le always contracts to 'au' — 'à le' is never correct.", audio: "n1u10-q01" },
      { type: "choice", q: "'I'm going' is...", choices: ["Je vais", "Je vas", "Je aller", "J'vais"], answer: 0, explain: "'Aller' is irregular: je vais [vɛ].", audio: "n1u10-q02" },
      { type: "choice", q: "'To the bathrooms' (plural) is...", choices: ["aux toilettes", "à les toilettes", "au toilettes", "à la toilettes"], answer: 0, explain: "à + les contracts to 'aux'.", audio: "n1u10-q03" },
      { type: "choice", q: "'You (formal) go' is...", choices: ["Vous allez", "Vous aller", "Vous alleez", "Vous vas"], answer: 0, explain: "'vous allez' [ale] — don't add an extra e.", audio: "n1u10-q04" },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u10-q05", transcript: "Où est la gare ?", choices: ["Où est la gare ?", "Où est la garde ?", "Où est le gare ?", "Où es la gare ?"], answer: 0, explain: "'gare' [gaʁ] vs 'garde' [gaʁd] — hear whether the final d sounds." },
      { type: "listening", q: "Listen and choose what you hear.", audio: "n1u10-q06", transcript: "Allez tout droit.", choices: ["Allez tout droit.", "Allez tout à droite.", "Allez à gauche.", "Allez tout droit ?"], answer: 0, explain: "[tu dʁwa] (straight) vs [a dʁwat] (to the right)." }
    ]
  }
];
