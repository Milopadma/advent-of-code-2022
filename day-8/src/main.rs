fn main() {
    // input (tree heights)
    let raw_input_tree_heights =
        "012112312021432444002222553400425232521024405364230243502011133453405140031345502304422434041110301
    200232301433000032014553313430051451345415452644031612133360315125411252342150441444110303303220232
    012032043131124023143415305334231554340040026525644646223162363631532034552251555300242421333001202
    010020134104321222122223340545001460560633343261605434552310604232663005103225352424310114240422011
    212021301001244100000352404441640645532211525220441015016510634253146202235514043015541322410011201
    132144121443420135115245431035255633645640610660540156430232462231111000410252255510232201044200222
    023122033142412052555452511231422534446011626400353220402440031412301615233502121030511512442321010
    301141120110543404532111655463262206202552235464332251306056524500625025456202233520422004221132302
    331123202210342524105321304411661214033027742766717261533332512451612643623061645055142443110324301
    221313132445123144414545615244355366141571136543235637457377213643014624434565365422334341413344243
    221242111311445543001352156316105017576561775627652666752474422667752314425542013000021341233100024
    404240040341235132000145356616223143731157164574444232751116317126316033411025323332110255240003212
    123003330203143102533436263112371637112636144711457534736745162573522252634166522262055041344324030
    431411133013330051242262640677115133664266254563711175515544551357544542224461364250555340313513440
    041045323040503342360554421644755312244721724142263853517671172161561221623610123506445500522353340
    301403055431320245405044136116554711363211222665635787542547661153466257723256100543450525310212113
    431302254541050260353365553137772261536525666642686332588662563477774743644535342435536313542442501
    012153025053104625362044225353653755562336676536424257644372586747613642724541332553631310301240412
    310253313543226544503515732355237677475822777637454834764364828843366131556552534635162131034353133
    444241431332240506455327437514758783478554764587677473444456467855476624766225114466405642453234152
    402213240104116302326611113321482347277847266753753328855526567783766822535162412510643530113335522
    000230020440366650656124162175787657426727568864564478376747264388773477653177577570641001263300000
    422035116451226140154255566263762343747662436795945698778367536563573487655316455773636464411125300
    015151043234310404774461424537647843866259456455999773694533746368577638877337423435664651136153050
    002054124312152414313331725826635828447948944935833394374894356635336455337422363277636513213124443
    314402204340263661133121578364627427755946449836357753459565649573663876245574716223510313546012413
    210031244166151467775466585322575686774534974657995663354697665884457645868244165667755101452162211
    415214660564621762331117623756335387868638678738476684669667679633846843535637636634211050441312021
    454300564411665133474437374872657476954688944438376988765884485796883722426832637554571726513224243
    514356353051475325736845534342255887357964368585965784845596876568349467873856371321334612541260502
    323024321045245371537845468837898756959838846845497969788686877865384463567434853235774711625350330
    232105241253721117313455325466683684759557584566667976445956954843896893433778727526275171462544452
    451153033507334751373345868344564679574775798468578574899454987348439597448584288641521453416313623
    004465233512212727587688727559368548969887587797486847994865479883857749674246853317464665306143063
    320405610017246264758775563646964644547557544574958869784669659675587365485833334886143532666353510
    116036510025731272226442238587474934585746977894476644795667774785693883646745268556124333522633102
    211302226132741615444228856433337599984764658798675588898559665864546374678843257433566761431541322
    410125210226647273775682736553839587587445677699879789997784899468494653455935578688564511622230414
    200065124272327765435485763433367889447776795766879598658886698686894484333384568255362146621146210
    214342332164714437377676854344334995587497657566867665665856875997876738846965447746817426272042312
    203166515652674728477284643588467998757595567656797998676676799457954466858367262737216155721366622
    426600321376436568373837878946475496497959659987995999886656566964465485646545567875744223231220533
    306442664253667257235359689888697985656588968888758596799666785665845993894659686636375146657052354
    056063152721142767882339937974695965586895779966969788688958567865775977955474258264555224716430462
    121216416714643278625435993466956887776697889968787868958775795755546749566674874882866614344323255
    343030602476553334722499484474678778766856679878889698686785688979955846334959678668744452627044516
    136163153755567822428495586547889696895679997679998879889575685898745568487778526443386326727265444
    012503133426363882745594755648686759687969987768966696679975799668465486369668923858621552561633354
    046531552324425637682688663936864498979555776669969869689875979857666774668553774357246143451746504
    226653636773753754465559863468786544757978668989989978868886768559884999489644378538675154361724244
    566245243513264228568389773557468969585856979866988776868695788877848949766894353728682212453106112
    101040543143746675857563375498466868659977567689967699867968667994975996764566482487668461744700644
    344464371655427323278247689649845468979669768976779979867878656996676565846664385347748662727161551
    222401354212461636755794664784945867878897697769868969689767668875475994595374733663747613773342436
    043643042317517272555283439656965646897987978887998768967787869765499444989766948472344176326302302
    113033456464766552664468878834959565595875999767977697799689858669557476397795387262772443652331352
    045356265677122434528587783596899854765798588769668799698889895758666569943796483248752765264556524
    523541132521626622555525537933999965948686757795689998897987956895794989654394736438222743252646522
    400362321547527466785487985775465775898666578577877556887696776879964544385549437742837775762312334
    126012317262423752744858788458355667645789966899775599576578778669559849784839668827631154462131442
    554322537362653247473654935749878845868856665565679857998675675875886798944898678765312724314306616
    120103404153525487337336474544599976946989875556795666866788679947664399694777328584266334612041510
    446305161552267328367627763887449557784966879665788687797898956965576645949632455885215315145625650
    250213422361246546576682883884767968848898986868565975796554458967966898746457785488145361645316553
    025504010222157234825655354433996594764659575488688665474594784467959397475434476473154224436053566
    233110203414657754375535729795368397677485866944885498564789948798679989674642454526453137614525150
    355122201244675552448366853547698574846657545698898478646777865943877977449856375572715145760310121
    351014655625447715762835572769548484476655676657899784457878764884355577975282757312643345315615352
    402121302116763611426738433596945987975559775866864988955455765836967845926767485335354566151120300
    034443214416241133773652822267867783934657477678464778895697543374844753638866475364536564535126313
    221511362303762643514656785468834977688586746569954745755957674853644636852327362271444270215510242
    422314522130575113436883866723368736745885875696884697963389344385585476878488273251213324544346400
    031240304140244334722423872537644835373684394344358877434953468748376265428586577622162546625355005
    132105021261022653353117844644834476375567763835458437655789897367636223634463143667735112026324132
    514415554451167143173274235227534379387894638396894756689438733847484822835475766375123631360405331
    025354124234323175277452152324826778349776865877538793348483367594372728273531627545332511505301213
    453352255300162565575416757678335357789953554755878483336896596667767755665177622411725150631315100
    413103154011101367542667323677856676488374676666836437789558854642447354463535552173434520533012441
    450104240150022303775122463726353226847482868748648735455774843564588623712664726721062216253413232
    254401130161531010321572735316568855666423738543444586863466335232385554443435111113351051013332243
    012015524153160101056234475254786533563762562375668764824655756554774765175134152142641636211224101
    401513200016421444107742514152325367337752834378745856682486453572625244243554234234666300133321122
    445314540322466420344527723145114522663468775224667885568744767758861741156565275625430112114112044
    243403024520642150244454235212413671226763478557552454828363832654761153116273660316623125220255514
    301220513003265612655251443325734461356485624255663233883258884623715411454764055601331324550241041
    214434141032520063620263512123645237123562522355332572858724267675167431243325125535523333013544101
    224112504414005414365110215744441552326452751234372242326342251175365565425526625116413015324441234
    101244504131045203015440021227131367664315733652152525611111756764732525302602511566425222200011414
    443311333410300035451560503132315523677334745266743437414451622745117432363401635464232345313320114
    024223422212215325314532533211424726755463757376376374436415341673574653605032513025214443124210423
    131041000202140522542363541314122073523112152365557225641131412673651044451242262154015004224423032
    003202443245514132234432305024102244276736546316472356265253226350555410534512000145434044243113241
    313413132144313154425240554404300310630316746374151477261424623614044300020406525441152024431020402
    103411141120335202112330241356400601060103443256553657151625431221643156223415021354512251132320300
    332444003204211304141152222516224013253024022231546665002306403026526355544151105553204112124302430
    321213340322433455001512004216166134154564156653332542412540461601625556352235340402131413232413301
    321031332121142023421142333135265455265161160360524216336041155533134560550234435134012231312131101
    321113020222222432421215451340550656020623203643661060316623240214424224523310151045300422023311331
    231000331134244320342144235314144416543360505663262655442043222411240015433301222112313320311210030";

    // each number in the input is a tree height, being 0 the shortest, and 9 the tallest
    // a tree is visible if all the other trees between it and an edge of the grid are shorter than it,
    // only considering trees of the same column or row.
    // each new line is a new row, and each number is a new column

    // the input is a string, so we need to split it into lines
    let lines = raw_input_tree_heights.split('\n').collect::<Vec<&str>>();

    // we need to convert the strings into numbers
    let mut tree_heights = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c.to_digit(10) {
                Some(_) => { c.to_digit(10).unwrap() }
                None => { 0 }
            });
        }
        tree_heights.push(row);
    }

    // at this point, the input is a 2D vector of numbers
    // we can now iterate over the vector and count the visible trees
    let mut visible_trees = 0;
    // find out how many trees are  on the edges of the grid
    for row in 0..tree_heights.len() {
        for col in 0..tree_heights[row].len() {
            // if the tree is on the edge, it's visible
            if
                row == 0 ||
                row == tree_heights.len() - 1 ||
                col == 0 ||
                col == tree_heights[row].len() - 1
            {
                visible_trees += 1;
            } else {
                // if the tree is not on the edge, we need to check if it's visible
                // we need to check the trees in the same row and column
                // we can do this by iterating over the trees in the same row and column
                // and checking if they are taller than the current tree
                let mut visible = true;
                for i in 0..tree_heights.len() {
                    if tree_heights[row][i] > tree_heights[row][col] {
                        visible = false;
                        break;
                    }
                }
                if visible {
                    for i in 0..tree_heights[row].len() {
                        if tree_heights[i][col] > tree_heights[row][col] {
                            visible = false;
                            break;
                        }
                    }
                }
                if visible {
                    visible_trees += 1;
                }
            }
        }
    }

    println!("visible trees: {}", visible_trees);
}