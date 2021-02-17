use crate::{clamp_to_u8, render_sdf, BitmapGlyph};

#[test]
fn test_empty_glyph_unbuffered() {
    // Tests an empty glyph. In this case, we are using the actual bitmap (empty) and metrics
    // for how Open Sans Light encodes a space (0x20).
    let data = Vec::new();
    let bitmap = BitmapGlyph::new(data.clone(), 0, 0, 0);
    let sdf = render_sdf(&bitmap, 8);

    assert_eq!(sdf, Vec::new());
    assert_eq!(clamp_to_u8(&sdf, 0.25), data);
}

#[test]
fn test_empty_glyph_buffered() {
    // Tests an empty glyph. In this case, we are using the actual bitmap (empty) and metrics
    // for how Open Sans Light encodes a space (0x20), plus a 3px buffer we added.
    let data = Vec::from([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ]);
    let bitmap = BitmapGlyph::new(data.clone(), 0, 0, 3);
    let sdf = render_sdf(&bitmap, 8);
    let sdf_data_f64 = Vec::from([
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    ]);

    assert_eq!(sdf, sdf_data_f64);
    assert_eq!(clamp_to_u8(&sdf, 0.25), data);
}

#[test]
fn test_nontrivial_glyph() {
    // Tests an nontrivial glyph. In this case, we are using the actual bitmap and metrics
    // for how Open Sans Light encodes an ampersand (0x25), plus a 3px buffer we added.
    let alpha = Vec::from([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27, 75, 91, 55, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 3, 141, 246, 196, 180, 231, 205, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        117, 236, 42, 0, 0, 10, 180, 207, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 203, 130, 0, 0,
        0, 0, 46, 255, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 215, 107, 0, 0, 0, 0, 33, 255,
        31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 169, 0, 0, 0, 0, 113, 221, 1, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 247, 71, 0, 0, 65, 240, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 96, 241, 69, 138, 231, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8,
        199, 255, 191, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 222, 186, 136, 242,
        58, 0, 0, 0, 0, 0, 8, 32, 0, 0, 0, 0, 0, 0, 0, 0, 85, 240, 92, 0, 0, 117, 241, 56, 0, 0, 0,
        0, 105, 175, 0, 0, 0, 0, 0, 0, 0, 17, 240, 79, 0, 0, 0, 0, 118, 240, 54, 0, 0, 0, 199, 81,
        0, 0, 0, 0, 0, 0, 0, 100, 210, 0, 0, 0, 0, 0, 0, 118, 239, 52, 0, 73, 230, 6, 0, 0, 0, 0,
        0, 0, 0, 134, 173, 0, 0, 0, 0, 0, 0, 0, 119, 238, 64, 226, 84, 0, 0, 0, 0, 0, 0, 0, 0, 123,
        196, 0, 0, 0, 0, 0, 0, 0, 0, 120, 255, 182, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 252, 45, 0, 0,
        0, 0, 0, 0, 7, 163, 230, 235, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 230, 92, 4, 0, 0, 14, 90,
        218, 189, 17, 120, 235, 44, 0, 0, 0, 0, 0, 0, 0, 0, 3, 118, 231, 251, 222, 227, 251, 197,
        91, 2, 0, 0, 125, 233, 43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 34, 55, 49, 17, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);
    let sdf_data_f64 = Vec::from([
        0.802713889938592,
        0.7097355768878618,
        0.6279726021892563,
        0.562338500460662,
        0.5189890067239825,
        0.4533784415316577,
        0.39834283129695547,
        0.3782221718060362,
        0.3758820364999128,
        0.37542659639029446,
        0.3766802833326899,
        0.3800127131415124,
        0.4000433253401104,
        0.4548732374510215,
        0.5338863756916572,
        0.6265379482182049,
        0.7084665133635307,
        0.8015920412263822,
        0.9024548745269637,
        1.0,
        1.0,
        1.0,
        0.7314195711767262,
        0.6279726021892563,
        0.5338301125829696,
        0.4548071999214019,
        0.39996823511417257,
        0.3569692020968682,
        0.28381686215881313,
        0.25480779274911264,
        0.25132112001047946,
        0.25063944078536615,
        0.252513436972363,
        0.25745807843059304,
        0.2861986410680061,
        0.35886579963709747,
        0.45282424908288416,
        0.5321417109731936,
        0.6265379482182049,
        0.7301881953013742,
        0.8396724364640523,
        0.9529820567867361,
        1.0,
        1.0,
        0.6759064943469222,
        0.562338500460662,
        0.4548071999214019,
        0.35878209138744094,
        0.2860936718984641,
        0.25734138629522074,
        0.18351297296288002,
        0.13435777329829018,
        0.12762172763021898,
        0.126274024554542,
        0.1299539758976059,
        0.13931856354834196,
        0.18717548490433622,
        0.28293073455790885,
        0.35626507063909296,
        0.45282424908288416,
        0.5607359454836812,
        0.6745737917807643,
        0.7917858299802276,
        0.9026801066556976,
        1.0,
        1.0,
        0.6402925808568658,
        0.5189890067239825,
        0.39996823511417257,
        0.2860936718984641,
        0.1870149435214898,
        0.13910280047628812,
        0.125,
        0.04926470588235294,
        0.025735294117647058,
        0.017892156862745097,
        0.03553921568627451,
        0.06151960784313726,
        0.1324756602454877,
        0.18213950850235072,
        0.2829307345579089,
        0.39771195676956694,
        0.5172521634149807,
        0.6268623253569651,
        0.7304665460867743,
        0.839914504549089,
        0.9531953498375564,
        1.0,
        0.6279726021892563,
        0.5037108189232647,
        0.3799336640788047,
        0.25734138629522074,
        0.13910280047628812,
        0.06102941176470588,
        -0.006617647058823534,
        -0.05808823529411765,
        -0.033578431372549025,
        -0.025735294117647065,
        -0.05073529411764706,
        -0.037990196078431376,
        0.04387254901960784,
        0.1324756602454877,
        0.2538204100490697,
        0.35683522100815884,
        0.45327295854919647,
        0.5610983647738974,
        0.6748750809979145,
        0.7920425335497719,
        0.9112937917883241,
        1.0,
        0.6250211934122977,
        0.5000264915127326,
        0.37503532128925254,
        0.25005297881555594,
        0.12510592397857453,
        0.005147058823529414,
        -0.05318627450980393,
        0.04191176470588236,
        0.125,
        0.125,
        0.05759803921568628,
        -0.025735294117647065,
        -0.03897058823529412,
        0.125,
        0.18325221677224415,
        0.28364832971822973,
        0.39822277050909777,
        0.5176450279409062,
        0.6392037038002374,
        0.7618768765042955,
        0.8852013188828526,
        1.0,
        0.625,
        0.5,
        0.375,
        0.25,
        0.125,
        -0.03700980392156862,
        -0.001225490196078427,
        0.125,
        0.18167717528837035,
        0.1859234630741686,
        0.13122911580245636,
        0.03995098039215686,
        -0.125,
        0.0482843137254902,
        0.13400139906710512,
        0.2546200599951653,
        0.3780957219434538,
        0.502325964839507,
        0.6268623253569651,
        0.7515526428347794,
        0.8398312478672554,
        0.9026026395342922,
        0.625,
        0.5,
        0.375,
        0.25,
        0.125,
        -0.0428921568627451,
        0.010049019607843136,
        0.12540328063921938,
        0.2502018840757975,
        0.2542555198558384,
        0.13330742431373666,
        0.04632352941176471,
        -0.125,
        0.04730392156862745,
        0.1336512663455564,
        0.2544359663958122,
        0.377971772749991,
        0.5022326761529669,
        0.6267875724643643,
        0.6747714612328173,
        0.730370813282042,
        0.8017583955870193,
        0.625,
        0.5,
        0.375,
        0.25,
        0.125,
        -0.01740196078431372,
        -0.020343137254901958,
        0.125,
        0.1789331516530035,
        0.1794119321224977,
        0.12520192264523664,
        0.0071078431372549045,
        -0.04583333333333334,
        0.06200980392156863,
        0.1395357150782243,
        0.25757565060073395,
        0.3800923779588212,
        0.5021867430491086,
        0.5175099273388593,
        0.5609737292371823,
        0.626750767765203,
        0.7086547289719243,
        0.6263710931608822,
        0.5017128126204857,
        0.3772807261808621,
        0.25340826021966734,
        0.13168426765395566,
        0.04142156862745098,
        -0.05857843137254902,
        0.027696078431372548,
        0.12803153033716633,
        0.125,
        0.030637254901960786,
        -0.05514705882352941,
        0.027696078431372548,
        0.12803153033716633,
        0.18733717138462241,
        0.28630441104249754,
        0.3795476684450432,
        0.37791073667504,
        0.3980471390354055,
        0.4531186653562965,
        0.5323922659977992,
        0.626750767765203,
        0.6253471258251514,
        0.5170500424016602,
        0.39744904874406056,
        0.2825610488860038,
        0.18156471669230956,
        0.12595010889552993,
        0.015441176470588236,
        -0.05563725490196078,
        0.028676470588235296,
        -0.005147058823529407,
        -0.05073529411764706,
        0.027696078431372548,
        0.12803153033716633,
        0.1789331516530035,
        0.28087732688929656,
        0.2855808687956328,
        0.2567711678169269,
        0.25434528675458373,
        0.28340170234892986,
        0.3566392082963838,
        0.4531186653562965,
        0.5609737292371823,
        0.5330891368033847,
        0.4511751630772441,
        0.3541666666666667,
        0.28028383431403564,
        0.17984668836227952,
        0.12930518672844835,
        0.05857843137254902,
        -0.03504901960784314,
        -0.125,
        -0.031127450980392157,
        0.0517156862745098,
        0.12803153033716633,
        0.1789331516530035,
        0.28087732688929656,
        0.2855808687956328,
        0.18622951597979423,
        0.13804503838265408,
        0.13347855593417032,
        0.18287024059226104,
        0.28340170234892986,
        0.3980471390354055,
        0.5175099273388593,
        0.45393725092547516,
        0.3576786655334335,
        0.28028383431403564,
        0.17800007802744855,
        0.1267242193812129,
        0.03308823529411765,
        -0.04632352941176471,
        -0.02867647058823529,
        -0.004166666666666666,
        -0.05612745098039215,
        0.034068627450980396,
        0.12955952831186787,
        0.18021773990223874,
        0.2816974152800678,
        0.2567711678169269,
        0.13804503838265408,
        0.05857843137254902,
        0.04681372549019608,
        0.13347855593417032,
        0.25434528675458373,
        0.37791073667504,
        0.5021867430491086,
        0.398978730984219,
        0.28470867176427517,
        0.18488923110278158,
        0.1267242193812129,
        0.020833333333333336,
        -0.05514705882352941,
        0.017401960784313726,
        0.125,
        0.125,
        0.005147058823529414,
        -0.05563725490196078,
        0.03504901960784314,
        0.12982077559262606,
        0.18041097115228533,
        0.25024317757708286,
        0.1254856482785001,
        0.011029411764705885,
        -0.023284313725490197,
        0.125,
        0.25,
        0.375,
        0.5,
        0.3788918417936414,
        0.2558007579695138,
        0.13623152270226513,
        0.05416666666666667,
        -0.05514705882352941,
        0.023774509803921567,
        0.12620550003521555,
        0.1776311578500202,
        0.17685161072077596,
        0.1250867154042614,
        0.0046568627450980365,
        -0.05514705882352941,
        0.03602941176470588,
        0.13008888696622292,
        0.17878402583317682,
        0.125,
        -0.03504901960784314,
        0.022794117647058826,
        0.12706129150653223,
        0.25103699289010767,
        0.3756921236854826,
        0.5005193021246114,
        0.37524221640522115,
        0.25036317814867026,
        0.12572478265124504,
        0.013480392156862746,
        -0.04044117647058823,
        0.125,
        0.1783682351665138,
        0.2800496888752758,
        0.27954728825482583,
        0.1768380229776014,
        0.1250867154042614,
        0.0046568627450980365,
        -0.05465686274509804,
        0.03700980392156863,
        0.12782303349990576,
        0.026715686274509806,
        -0.05024509803921569,
        0.05955882352941176,
        0.13846390670571018,
        0.2569966020401974,
        0.3797002152490931,
        0.5035347589394477,
        0.375,
        0.25,
        0.125,
        -0.003186274509803924,
        -0.022303921568627455,
        0.125,
        0.25,
        0.3543518411359207,
        0.3535840584226425,
        0.27954728825482583,
        0.1768380229776014,
        0.12506942516503028,
        0.004166666666666666,
        -0.05416666666666667,
        0.031127450980392157,
        -0.04828431372549019,
        0.021323529411764706,
        0.12680572899744869,
        0.1865402194171745,
        0.28578357801001725,
        0.3997464864888841,
        0.518818131391153,
        0.3750064878331507,
        0.25000973164450024,
        0.12501946215271853,
        0.0022058823529411756,
        -0.033578431372549025,
        0.125,
        0.181343565516743,
        0.2572255753923184,
        0.2859895044156503,
        0.2849921124049326,
        0.2561161926412029,
        0.17682579311602456,
        0.12505405405658052,
        0.003676470588235295,
        -0.125,
        -0.026715686274509806,
        0.125,
        0.17805811665457544,
        0.28032069653626074,
        0.3559148455160962,
        0.4525487567751641,
        0.5319452475091385,
        0.37672684867435546,
        0.25258289433789993,
        0.13008888696622292,
        0.03602941176470588,
        -0.06102941176470589,
        0.04044117647058823,
        0.12620550003521555,
        0.1388884323329674,
        0.18685555018759617,
        0.1853254006687312,
        0.1368228933074643,
        0.12634441380470282,
        0.05906862745098039,
        -0.01740196078431372,
        -0.05024509803921569,
        -0.05269607843137254,
        0.03995098039215686,
        0.13122911580245636,
        0.1814535126657697,
        0.2824896055764648,
        0.35597155272234693,
        0.452593356499583,
        0.396923315656955,
        0.28182107535120704,
        0.18041097115228533,
        0.125,
        -0.015441176470588236,
        -0.05024509803921569,
        0.017401960784313726,
        0.06053921568627451,
        0.125,
        0.125,
        0.05563725490196078,
        0.018382352941176468,
        -0.04436274509803921,
        -0.030147058823529416,
        0.05416666666666667,
        0.003676470588235295,
        -0.05269607843137254,
        0.04093137254901961,
        0.13153089849440944,
        0.18156471669230956,
        0.2825610488860038,
        0.39744904874406056,
        0.45213174906448533,
        0.35538446577208566,
        0.25734138629522074,
        0.13910280047628812,
        0.06102941176470588,
        0.0046568627450980365,
        -0.05073529411764706,
        -0.0605392156862745,
        -0.04632352941176471,
        -0.04877450980392156,
        -0.0605392156862745,
        -0.034068627450980396,
        0.017892156862745097,
        0.06151960784313726,
        0.13623152270226513,
        0.12500600716053883,
        0.001225490196078434,
        -0.0517156862745098,
        0.04142156862745098,
        0.13168426765395566,
        0.25340826021966734,
        0.3772807261808621,
        0.5189890067239825,
        0.39996823511417257,
        0.2860936718984641,
        0.1870149435214898,
        0.13910280047628812,
        0.1250867154042614,
        0.06200980392156863,
        0.04583333333333334,
        0.03553921568627451,
        0.03848039215686275,
        0.05416666666666667,
        0.125,
        0.126274024554542,
        0.13931856354834196,
        0.18717548490433622,
        0.17678094305162162,
        0.12500600716053883,
        0.125,
        0.13168426765395566,
        0.18156471669230956,
        0.2825610488860038,
        0.39744904874406056,
        0.562338500460662,
        0.4548071999214019,
        0.35878209138744094,
        0.2860936718984641,
        0.25734138629522074,
        0.18733717138462241,
        0.1395357150782243,
        0.13313787757225382,
        0.1299539758976059,
        0.13078891612268198,
        0.13623152270226513,
        0.18488923110278158,
        0.25063944078536615,
        0.25745807843059304,
        0.2861986410680061,
        0.27951118372297856,
        0.25000300363439776,
        0.25,
        0.25340826021966734,
        0.2825610488860038,
        0.35597155272234693,
        0.452593356499583,
        0.6279726021892563,
        0.5338301125829696,
        0.4548071999214019,
        0.39996823511417257,
        0.35895015779686096,
        0.28630441104249754,
        0.25757565060073395,
        0.25416666666666665,
        0.252513436972363,
        0.25294414517941693,
        0.2558007579695138,
        0.28470867176427517,
        0.3576786655334335,
        0.3800127131415124,
        0.4000433253401104,
        0.3952866071930855,
        0.3750020024296146,
        0.375,
        0.3772807261808621,
        0.39744904874406056,
        0.452593356499583,
        0.5319452475091385,
        0.7097355768878618,
        0.6279726021892563,
        0.562338500460662,
        0.5189890067239825,
        0.45493979357975645,
        0.4001190020261364,
        0.3800923779588212,
        0.3777905430849804,
        0.3766802833326899,
        0.37696915070141473,
        0.3788918417936414,
        0.398978730984219,
        0.4539372509254751,
        0.5037704458869867,
        0.519046878566063,
        0.5153896601855926,
        0.5000015018239652,
        0.5,
        0.5017128126204857,
        0.5170500424016602,
        0.5605495039223196,
        0.6263710931608822,
    ]);
    let sdf_data_u8 = Vec::from([
        0, 10, 31, 47, 58, 75, 89, 94, 95, 95, 95, 94, 89, 75, 55, 31, 10, 0, 0, 0, 0, 0, 4, 31,
        55, 75, 89, 100, 118, 126, 127, 127, 126, 125, 118, 99, 75, 55, 31, 5, 0, 0, 0, 0, 18, 47,
        75, 99, 118, 125, 144, 156, 158, 159, 158, 155, 143, 119, 100, 75, 48, 19, 0, 0, 0, 0, 27,
        58, 89, 118, 143, 155, 159, 178, 184, 186, 182, 175, 157, 144, 119, 89, 59, 31, 4, 0, 0, 0,
        31, 62, 94, 125, 155, 175, 192, 206, 199, 197, 204, 200, 180, 157, 126, 100, 75, 48, 19, 0,
        0, 0, 31, 63, 95, 127, 159, 189, 204, 180, 159, 159, 176, 197, 201, 159, 144, 118, 89, 59,
        28, 0, 0, 0, 31, 63, 95, 127, 159, 200, 191, 159, 144, 143, 157, 181, 223, 178, 157, 126,
        94, 63, 31, 0, 0, 0, 31, 63, 95, 127, 159, 202, 188, 159, 127, 126, 157, 179, 223, 179,
        157, 126, 94, 63, 31, 19, 5, 0, 31, 63, 95, 127, 159, 195, 196, 159, 145, 145, 159, 189,
        202, 175, 155, 125, 94, 63, 59, 48, 31, 10, 31, 63, 95, 126, 157, 180, 206, 184, 158, 159,
        183, 205, 184, 158, 143, 118, 94, 94, 89, 75, 55, 31, 31, 59, 89, 119, 144, 159, 187, 205,
        183, 192, 204, 184, 158, 145, 119, 118, 125, 126, 118, 100, 75, 48, 55, 76, 100, 119, 145,
        158, 176, 200, 223, 199, 178, 158, 145, 119, 118, 143, 156, 157, 144, 118, 89, 59, 75, 100,
        119, 145, 158, 182, 203, 198, 192, 205, 182, 158, 145, 119, 125, 156, 176, 179, 157, 126,
        94, 63, 89, 118, 144, 158, 185, 205, 186, 159, 159, 189, 205, 182, 158, 145, 127, 159, 188,
        197, 159, 127, 95, 63, 94, 126, 156, 177, 205, 185, 159, 145, 146, 159, 190, 205, 182, 158,
        145, 159, 200, 185, 158, 127, 95, 63, 95, 127, 159, 187, 201, 159, 145, 119, 119, 146, 159,
        190, 205, 181, 158, 184, 204, 176, 155, 125, 94, 62, 95, 127, 159, 192, 196, 159, 127, 100,
        101, 119, 146, 159, 190, 205, 183, 203, 185, 158, 143, 118, 89, 58, 95, 127, 159, 190, 199,
        159, 145, 125, 118, 118, 125, 146, 159, 190, 223, 198, 159, 145, 119, 100, 75, 55, 95, 126,
        158, 182, 206, 180, 159, 155, 143, 143, 156, 159, 176, 195, 204, 204, 181, 157, 144, 119,
        100, 75, 90, 119, 145, 159, 195, 204, 186, 175, 159, 159, 177, 186, 202, 198, 177, 190,
        204, 180, 157, 144, 119, 89, 75, 100, 125, 155, 175, 190, 204, 206, 203, 203, 206, 199,
        186, 175, 156, 159, 190, 204, 180, 157, 126, 95, 58, 89, 118, 143, 155, 159, 175, 179, 182,
        181, 177, 159, 159, 155, 143, 146, 159, 159, 157, 144, 119, 89, 47, 75, 99, 118, 125, 143,
        155, 157, 158, 157, 156, 144, 127, 125, 118, 119, 127, 127, 126, 119, 100, 75, 31, 55, 75,
        89, 99, 118, 125, 126, 126, 126, 126, 118, 100, 94, 89, 90, 95, 95, 95, 89, 75, 55, 10, 31,
        47, 58, 75, 89, 94, 94, 95, 95, 94, 89, 75, 62, 58, 59, 63, 63, 63, 59, 48, 31,
    ]);
    let bitmap = BitmapGlyph::new(alpha, 16, 19, 3);
    let sdf = render_sdf(&bitmap, 8);

    assert_eq!(sdf, sdf_data_f64);
    assert_eq!(clamp_to_u8(&sdf, 0.25), sdf_data_u8);

    // Sanity check on the clamp_to_u8 function; 191 = 255 (max value of a u8) - 25% of 256 (range)
    assert_eq!(
        clamp_to_u8(&sdf, 0.25)
            .iter()
            .filter(|x| **x >= 191)
            .count(),
        sdf_data_f64.iter().filter(|x| **x < 0.0).count()
    );
}