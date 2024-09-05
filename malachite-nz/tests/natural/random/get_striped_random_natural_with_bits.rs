// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::num::random::striped::StripedBitSource;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::random::get_striped_random_natural_with_bits;

fn get_striped_random_natural_with_bits_helper(
    m_numerator: u64,
    m_denominator: u64,
    bits: u64,
    out: &str,
) {
    let mut bit_source = StripedBitSource::new(EXAMPLE_SEED, m_numerator, m_denominator);
    let xs = (0..10)
        .map(|_| get_striped_random_natural_with_bits(&mut bit_source, bits))
        .collect_vec();
    assert_eq!(xs.to_debug_string(), out);
}

#[test]
fn test_get_striped_random_natural_with_bits() {
    get_striped_random_natural_with_bits_helper(2, 1, 0, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_striped_random_natural_with_bits_helper(2, 1, 1, "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]");
    get_striped_random_natural_with_bits_helper(
        2,
        1,
        10,
        "[716, 684, 662, 763, 798, 829, 768, 732, 536, 541]",
    );
    get_striped_random_natural_with_bits_helper(
        2,
        1,
        100,
        "[756308944479610176770360563916, 1145718678910746691112769802930, \
        1080305256857112995037586132048, 807717805879357681845918965159, \
        1215845349850185248466264185684, 822948728534233460050112643373, \
        955700757257715004518140113132, 832461439248077413553063350320, \
        1242643020761038901934578711610, 1122137987246906014371413743505]",
    );
    get_striped_random_natural_with_bits_helper(
        2,
        1,
        1000,
        "[1066036401822586582060831035662706457005124173369385364089512753873668954854498653201330\
        529188610070039476426288535963559977582497470497937854727434805905648395790056513373361685\
        117044701404743375692128198904783759739184161371638993268438163708872889304697920949732034\
        4797581420099450165301088751016140, \
        987176835967471470631208736450711102640000526855618567465986968908194747303665548783151326\
        849739186186452340671673297796771266911335225612017707433164734645560965513055402080032340\
        135474058189238938067890211197448581037553663122597099692488328028757994359299305393973140\
        7925926109130072336312534438574, \
        568651645319898625478110927429560819026449278119016593712297801674333004668766652853597774\
        772681158919761337797730830161503881200296844963400983419113636067755214296565242049691883\
        922630951084279967424027354828765778295400023859140245298770234805718721051701730000306553\
        5144184078571910290480368694380, \
        758830907466434071959666860937945102677771418528458236060516770128040285772163826421306107\
        069853863334698116336945744601463639747199523669413845254638599625980012214050809846459478\
        559098934176586233093238956124608709775991202493060441152475782277782012828793855484827036\
        1411929636489612968071552336405, \
        980856434133966113634431860230323947967355946268495564634504680787571319483353753543014805\
        474807358240303253563585951883147532327134463459185707026951902911405729653418676374715955\
        691096425012748194433199980744016228705422729019166704577441683479974798193714636957924636\
        2973260186276362799319175133568, \
        894303548113446750904572507746592793549624231872528102587605556722756026081514290865978856\
        949605256419586901475129581474267478481820707346973965338715875069463779465480362052962715\
        102197582459759242950488814694110771040964546445427112279563255714907257699939184423978433\
        0215727680884042630165644025099, \
        974492070270890205196098593733513997055807419178419835560661240335358848519651158952303927\
        871434236017718573182013896425860901259995476236242988456734111274812403199906058941973460\
        877801690192132645453156214170274198928980236695132799825665804217976367252593838779965803\
        0607916391896384659714182950628, \
        645130850234339503642007350857138479917399859387095109454794616985938391311231126744661800\
        420543794746031491748493862132931262497139684250480230073784453843645088983023718012240528\
        683824553139343195888019854863909746543574579361265915812174560716242126991285521565613433\
        2466523138255580920562084028294, \
        639103284871251403821872209988025331131792409726095776075819070688817857426736537831222024\
        179652645603869247406404689998157064676469146016517502338483799149248172262554878327272758\
        174240821091933100132108595935722179262833993962469175972706639648644071967476854878243149\
        4649776909489103783746916780024, \
        104643900952018601215342460716610794174551332701220396118082370352258784597041601116915384\
        664642853384078500349464320717455867749180262488843336424123164153150026841959758704744253\
        756151269545080362145067323770739682710146919991724449298137503388491656477667755306189744\
        24803845757396271996047726290589]",
    );

    get_striped_random_natural_with_bits_helper(10, 1, 0, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_striped_random_natural_with_bits_helper(10, 1, 1, "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]");
    get_striped_random_natural_with_bits_helper(
        10,
        1,
        10,
        "[1016, 992, 1016, 767, 512, 513, 1020, 512, 1016, 1023]",
    );
    get_striped_random_natural_with_bits_helper(
        10,
        1,
        100,
        "[950737912392312175425017102328, 1109194416875305772521657204608, \
        648685415698526143125877948414, 1248153346840921551430087606303, \
        831891172891058554467585867776, 1267650600228229401359297740927, \
        950739156735808420047682273024, 638776758039873373851420326912, \
        643690285327603686643074121664, 671543785829629858305590427647]",
    );
    get_striped_random_natural_with_bits_helper(
        10,
        1,
        1000,
        "[5358165738110281147468429156665582144998881353628318343872646956815751197999946897572878\
        278363096783827886911589242856640635533315179867213030979090927223140806482585564866175368\
        333598466336603271609976534104173527941609131651006242168470752742668268779440452153819210\
        944528718553525957103310205894648, \
        539935796843008525931501938016824825731989695976128079798746287896348626759778818662810517\
        524977343980885794548702209431866052161353917750190601629912511180434696813340457173208570\
        976449234006368601336057069020362424176301595236556802947569224730136326560449690301955186\
        3255474664584713584582999408640, \
        535754303694423899076935129343771055050259990680670440133208286589122737827790634967047773\
        700260648071343420497642383540669384404571214562700313160956733565914399913133443167360146\
        252087697860065532691284023259655546679447897828373290595492357586809871031621268591800199\
        5164641448720709752408544641022, \
        535815617678664673230583609293711626429039758128419938980167626717612472897746291934554185\
        557830869500024824265898785919682260921341384664997061547297321382527299885697935410512118\
        522746192907474498033052134885264654751340612094587360238549546738016850498598238219826282\
        5015681391047050153768125472767, \
        100656671761770967200809514408313411282491427879094391861152015262204431791194203727185921\
        026045038825035836776620646775627513583957795766207833861462739511509698818541011638004365\
        409989389341031372058994101918102834059849507520841606970888386863725907172131052360271284\
        55992101445905914436525739998208, \
        107149228544600635260212788419695526657434400429968676165634060825808436991969085650247279\
        650596133663598434482129668133099616635115467318010721467086518060004035373781135234642383\
        205702114144671525516391055691812453033688242812694582953965743513229924054172419365064495\
        42173396641692237011907449425727, \
        107150860693727448661794856915575585277857852731679120989819290845002348669070050613296455\
        734254692011706809018756806353158106462995138380031851002523766291502961737781576247555759\
        035185454502873048780305065230433793569807065690804749332353542464565681262222119583879742\
        48405282741635132994030678311166, \
        107150835221766756970390016554040088032382955997319930334779105385220088376881426554895494\
        540316762324972302835401791427607201669829855912974990784820455466307137336628823875387612\
        135057991945407277136587896193153151600433435164863822917290693842788118252951172684773421\
        70526384435574180443755489763328, \
        535754329539033410731721563380478204940555300525204412489285381291927939025074977709123312\
        566899344374925407384165980119188647505571183538659634788083559269291976584227032503722306\
        929007576563288991617997079930222315375169710335488132792623698751814518487754242895518611\
        5084271519939681346646991441912, \
        535882021406206416670177596158366769032342285663637541146888579315644330411450606808875554\
        132489833818740595777819456634963853487245325624796744100983558326561831449646070534648278\
        476260874658721825904285846269922085259057263667571525921657161024153045937468235049728709\
        2438657975290976181526386966515]",
    );

    get_striped_random_natural_with_bits_helper(11, 10, 0, "[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]");
    get_striped_random_natural_with_bits_helper(11, 10, 1, "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]");
    get_striped_random_natural_with_bits_helper(
        11,
        10,
        10,
        "[682, 842, 668, 853, 862, 853, 850, 852, 682, 853]",
    );
    get_striped_random_natural_with_bits_helper(
        11,
        10,
        100,
        "[1063803140432100403291953916586, 739462850133133817539732346028, \
        1056478660679155688751767049562, 848401573590247656481079600469, \
        1056375500189999764883937602218, 1053067879147533365448711514965, \
        1056381847001601296578889995602, 1056377112103581910626871170386, \
        1056375600983290506811020749482, 845074597043259864052961293645]",
    );
    get_striped_random_natural_with_bits_helper(
        11,
        10,
        1000,
        "[8929237967230103262890722824433509449612079777418572793068173743542860931264379536551467\
        111265297962015834728928882271330897607870697615306329567845176199860105934578611877105699\
        420361112596704086865552677150738773328713505835358160809074402075384139431403167870451367\
        242430551069260362892051555117738, \
        848277807033155886730751542077297974149911088007915913764780956681936845102711155704193533\
        266937165754352296914790126319848647014472476095156708927462195514313159177074602462717519\
        349357763529281370152392669738095746762025827796955823134607626270938290494658223672222158\
        9872232895273605128836243696298, \
        622256279237828710101116559940449011192577108759402537346355985105564415773311644474958256\
        085364963664247523046669028723331763456095716282707295047182502032978485452383285604942655\
        047145606332906731031323587018683298574131563687284576221383367233664334964305026386536472\
        7120990168388537700787439359354, \
        893011037585013007271439602956526972024707806627477311937832595287305307343362049808812372\
        867697432889627779970803417048478482882370003668394928455851950345620310470241544275541047\
        354242013397873563064543296718018003131524951605807454956556860189105716328731797732674596\
        4570601733436095071743763633493, \
        714337697986576114077307239016734314472779437227875536493177724277496357242071140467095878\
        368847036684840951410204114568419486398409897184525378525275704002748768137539106304824083\
        362036096433222952552023855753638427465829477635653011302870027782444701974201225532101310\
        2549204865443174861708377533098, \
        717129479744396200344184597459955775324063665344347322215427784911546292557185067124997476\
        300110608871594211005211148242437189627028361260715625731282067311558916993444457603008618\
        146383526538195175746094394556394170673628321139601154519945289775172095900192878319082649\
        5453320605634381639129320740181, \
        712943875305246933549449117163766092515737565613814773068912395757448666817082911895780847\
        326837660502284270497684787917665238088905925731656976408461765235806743263267139181463148\
        004431439157535144190810911719314309577264591245575456701925751321667869578190443322813012\
        0138648732293712628634064954698, \
        892880942227156526968373917452622274287516410008666161523034951090805186930790904719943631\
        023279485226392520031870194759074725254789440380488049235603947036487803727696933564597255\
        812777037968411285945062427246430458697274335300861633278805179074880855146539152265137509\
        2760519788829492124624878217898, \
        892945641881431108631973304223007197865627901236713933938944990987439087863487393850263204\
        926847861908008259701418716838801341585797849737533079786960203576656895262092537802767877\
        046297885380150024466056272755057031975057975145576287997024449574764798500660830182847012\
        5379718921376291465124979747542, \
        712949327932932700218621033804526985196291897563252301818755627637478824989877935042325873\
        291167136650626866176302303072095225803612799473142905508378017958979111144644566257625076\
        950893722575069628745776862688326045350363911106097574031678159885821711577125514250680212\
        9840859041388985930050461586805]",
    );
}
