### Uzdevuma apraksts
Pieņemsim, ka mums ir jāizveido filmu ar vairākām ainām. Katrā ainā ir jāuzņem vienam vai vairākiem aktieriem. Aktieris uzņemt tikai vienu ainu dienā. Šo aktieru algas tiek aprēķinātas pa dienām. Šajā problēmā aktieris tiek pieņemts darbā līdz laikam, kad aktieris nofilmē visas savas ainas. Piemēram, mēs nevaram pieņemt darbā aktieri pirmajā un trešajā dienā, bet ne otrajā dienā. Nomas perioda laikā aktieriem joprojām ir jāmaksā, pat ja viņi nav iesaistīti filmēšanā. Vienā dienā var tikt filmēta tikai viena aina. Talantu plānošanas mērķis ir samazināt aktieru kopējo algu, pielāgojot ainu secību. Dots aktieru skaits N. Katram aktierim tiek piešķirta alga dienā. Dots ainu skaits M. Katrā M ainā tiek definēts aktieru masīvs, kur tiek ietverti aktieru numuri, kas piedalās ainā.

### Identificētie objekti (Domeins)
- Aktieris {id: number, vārds: string, likme: number, pirmā diena kad nodarbināts: number, pēdējā diena kad nodarbināts: number}
- Aina {id: number, nosaukums: string, aktieru saraksts: Aktieris[]}
- Grafiks {Ainu saraksts: Aina[] <<MAINĀMAIS>>, aktieru saraksts: Aktieris[], izmaksas: number} <<RISINĀJUMS>>

### Plānotais ieviešamais algoritms
Late acceptance hillclimbing
Tomēr objektu definēšanas un izstrādes fāzē sanāca izveidot arī lokālā optimuma meklēšanas algoritmu (atrod apkaimi, ņem labāko no apkaimes), ko neizdzēsu bet paturēju, lai salīdzinātu rezultātus.

### Izstrādes process
Vispirms tika izveidots objects.rs fails, kurā tika definēti objekti, ar ko optimizators darbosies. 
Tad tika definētas palīgmetodes, ar ko optimizatoram darboties, no kurām svarīgākā ir calculate_cost, kas strādā kā novērtējums optimizatoram. Cena tiek aprēķināta summējot aktieru algas pa dienām, kad viņi tiek izmantoti ainās. Ja aktieris parādās pirmo reizi, tiek pieskaitīta viņa alga par šo dienu, bet turpmākajās reizēs tiek pieskaitīta viņa alga reizināta ar dienu skaitu, kas pagājis kopš viņa pēdējās uzstāšanās. (NOVĒRTĒŠANA)

Tālāk tika veidots optimizer.rs fails, kurā tika definēts optimizatora objekts, kurš veic optimizācijas procesu. Tajā sākumā "spēlējoties" ar rust vidi tika izveidota naivā lokālā optimuma programma, kas tika pārdēvēta par funkciju *local_optimization*. 

Funkcija *late_acceptance_hillclimbing*, kas arī ir tā, ko vēlējos izstrādāt par parametriem ņem atmiņas rindas garumu, optimuma atrašanas reižu daudzumu, kā arī ciklu daudzumu. Sākotnēji bija doma optimizatora procesu beigt, kad konkrēts optimums sasniegts vairāk nekā X reizes, bet praksē, lai gan tas strādāja ar maziem piemēriem, lielos piemēros tas prasīja pārāk daudz laiku, lai būtu sakarīgi pārbaudāms, tapēc tika ieviests ciklu daudzuma parametrs, kas apstādināja optimizatoru pēc noteikta daudzuma ciklu.

Cikla solis ir izvēlēties 2 nejaušas pozīcijas ainu secībā un tās samainīt vietām. (GĀJIENS) Tālāk tiek aprēķinātas šī risnājuma izmaksas, ja tās ir lielākas kā gan pēdējam risinājuma, gan pēdējam, kas ir atmiņas rindā, tad to "atmetam", ierakstot izmaksas atmiņā. Ja risinājums ir labāks kā pēdējais vai pēdējais rindā, tad to pieņemam. Aprēķinam un piefiksējam pašreizējo labāko izmaksu, kā arī vai šī izmaksa ir atkārtojusies. 

Rust īpatnību dēļ (ownership/borrowing) ir izveidotas papildus palīgfunkcijas, lai ainu objektus varētu bīdīt vienā objektā tos nedublējot, tā arī paātrinto programmas ātrdarbību. Tāpat ir iespēja norādīt vai programmu vēlas verbose vai nē.

### Testēšana
Tika izveidoti 3 manuāli piemēri, kur pats aprēķināju optimālās izmaksas, ko pārbaudīju arī uz optimizatora, kur abi varianti nonāca līdz tam. Papildus tam izveidoju arī random piemēru ģeneratoru.

Piemērs ar 4 ainām, kā var redzēt hillimbing ir ievērojami ātrāks (jo beidz nevis timeout bet gan tā dēļ, ka 3 reizes ir sasniedzis vienu un to pašu optimumu).
```
LABĀKAIS
---------------------
IZMAKSAS: 310, SECĪBA: [3, 2, 1, 4, ]

LABĀKAIS
---------------------
IZMAKSAS: 310, SECĪBA: [4, 1, 2, 3, ]
LOKĀLAIS: 310 cost, 1.767793ms time; HILLCLIMBGING: 310 cost, 197.103µs time;
```

Piemērā ar 6 ainām, kur ir viens īpaši dārgs aktieris jau parādās atšķirība starp lokālo un hillclimbing, kur hillclimbing ir gan ātrāks, gan iegūst labāku rezultātu izkāpjot no lokālā optimuma. Dažreiz gan gadās ka iestrēgst pie ne optimālā rezultāta.
```
LABĀKAIS
---------------------
IZMAKSAS: 650, SECĪBA: [1, 2, 4, 6, 3, 5, ]

LABĀKAIS
---------------------
IZMAKSAS: 590, SECĪBA: [1, 5, 6, 3, 2, 4, ]
LOKĀLAIS: 650 cost, 3.739351ms time; HILLCLIMBGING: 590 cost, 1.816415ms time;
```

Piemērā ar 6 ainām un 2 vienlīdz dārgiem aktieriem kuri pārklājas lokālais iestrēgst lokālā optimumā, bet hillclimbing algoritms pārsvarā tiek ārā no tā. Te gana sāk eksponenciāli palielināties laika izmaksas, kur tas prasa jau 6 reizes ilgāku laiku.
```
LABĀKAIS
---------------------
IZMAKSAS: 700, SECĪBA: [1, 2, 4, 5, 3, 6, ]

LABĀKAIS
---------------------
IZMAKSAS: 620, SECĪBA: [4, 2, 1, 5, 3, 6, ]
LOKĀLAIS: 700 cost, 3.116485ms time; HILLCLIMBGING: 620 cost, 18.293973ms time;
```

Testējot ar randomizēto sarakstu, vais nestrādā x reizes sasniegts viens un tas pats minimums, un programma apstājas ar timeout. Dažreiz hillclimbing iegūst labākus rezultātus kā lokālais optimums, bet ar ievērojami lielāku laika izmaksu. Atmiņas izmēra parametrs ievērojami ietekmē rezultātus, to vajag kalibrēt lai dabutu labākos (lielāks atmiņas garums maziem datiem dod siltākus rezultatus). Kopumā kamēr uzdevums ir salīdzinoši viegls, lokālā meklēšana ir optimālāka.
```
LOKĀLAIS: 66747 cost, 882.438787ms time; HILLCLIMBGING: 63330 cost, 61.26545902s time; (atmiņas garums 3, 50 ainas)
LOKĀLAIS: 70294 cost, 825.791223ms time; HILLCLIMBGING: 70351 cost, 55.277612015s time; (atmiņas garums 3, 50 ainas)
LOKĀLAIS: 83482 cost, 716.803883ms time; HILLCLIMBGING: 83254 cost, 57.600839732s time; (atmiņas garums 10, 50 ainas)
LOKĀLAIS: 68761 cost, 910.940007ms time; HILLCLIMBGING: 70821 cost, 58.458498358s time; (atmiņas garums 10, 50 ainas)
LOKĀLAIS: 60590 cost, 970.621584ms time; HILLCLIMBGING: 63678 cost, 60.233444642s time; (atmiņas garums 50, 50 ainas)
LOKĀLAIS: 78478 cost, 752.02829ms time; HILLCLIMBGING: 79854 cost, 52.965096355s time; (atmiņas garums 50, 50 ainas)
LOKĀLAIS: 813092 cost, 51.042472501s time; HILLCLIMBGING: 905602 cost, 5.066776ms time; (atmiņas garums 3, 200 ainas)
LOKĀLAIS: 817689 cost, 40.985648865s time; HILLCLIMBGING: 820675 cost, 211.489553926s time; (atmiņas garums 10, 200 ainas)
LOKĀLAIS: 775952 cost, 46.13186907s time; HILLCLIMBGING: 782305 cost, 200.047018976s time; (atmiņas garums 50, 200 ainas)
```

### Saite uz Github un palaišana
https://github.com/JekabsVanags/actor_optimizer.git
uz linux var uzinstalēt rust vidi, cargo run palaiž programmu
uz windows github repo ir pievienots .exe fails (neesmu pārbaudījis, bet vajadzētu strādāt)
