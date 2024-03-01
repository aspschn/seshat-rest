use serde_json::{json, Value};
use seshat::unicode::CodePoint;
use seshat::unicode::Ucd;
use seshat::unicode::props::Blk;
// use seshat::unicode::props::PropertyName;
use seshat::unicode::props::UnicodeProperty;
use seshat::unicode::UNICODE_VERSION;
use crate::versions::SESHAT_VERSION;


pub fn browse_blocks_api_v3() -> Value {
    let mut v: Vec<Blk> = vec![];

    v.push(Blk::Adlam);
    v.push(Blk::AegeanNumbers);
    v.push(Blk::Ahom);
    v.push(Blk::Alchemical);
    v.push(Blk::AlphabeticPf);
    v.push(Blk::AnatolianHieroglyphs);
    v.push(Blk::AncientGreekMusic);
    v.push(Blk::AncientGreekNumbers);
    v.push(Blk::AncientSymbols);
    v.push(Blk::Arabic);
    v.push(Blk::ArabicExtA);
    v.push(Blk::ArabicExtB);
    v.push(Blk::ArabicExtC);
    v.push(Blk::ArabicMath);
    v.push(Blk::ArabicPfA);
    v.push(Blk::ArabicPfB);
    v.push(Blk::ArabicSup);
    v.push(Blk::Armenian);
    v.push(Blk::Arrows);
    v.push(Blk::Ascii);
    v.push(Blk::Avestan);
    v.push(Blk::Balinese);
    v.push(Blk::Bamum);
    v.push(Blk::BamumSup);
    v.push(Blk::BassaVah);
    v.push(Blk::Batak);
    v.push(Blk::Bengali);
    v.push(Blk::Bhaiksuki);
    v.push(Blk::BlockElements);
    v.push(Blk::Bopomofo);
    v.push(Blk::BopomofoExt);
    v.push(Blk::BoxDrawing);
    v.push(Blk::Brahmi);
    v.push(Blk::Braille);
    v.push(Blk::Buginese);
    v.push(Blk::Buhid);
    v.push(Blk::ByzantineMusic);
    v.push(Blk::Carian);
    v.push(Blk::CaucasianAlbanian);
    v.push(Blk::Chakma);
    v.push(Blk::Cham);
    v.push(Blk::Cherokee);
    v.push(Blk::CherokeeSup);
    v.push(Blk::ChessSymbols);
    v.push(Blk::Chorasmian);
    v.push(Blk::Cjk);
    v.push(Blk::CjkCompat);
    v.push(Blk::CjkCompatForms);
    v.push(Blk::CjkCompatIdeographs);
    v.push(Blk::CjkCompatIdeographsSup);
    v.push(Blk::CjkExtA);
    v.push(Blk::CjkExtB);
    v.push(Blk::CjkExtC);
    v.push(Blk::CjkExtD);
    v.push(Blk::CjkExtE);
    v.push(Blk::CjkExtF);
    v.push(Blk::CjkExtG);
    v.push(Blk::CjkExtH);
    v.push(Blk::CjkExtI);
    v.push(Blk::CjkRadicalsSup);
    v.push(Blk::CjkStrokes);
    v.push(Blk::CjkSymbols);
    v.push(Blk::CompatJamo);
    v.push(Blk::ControlPictures);
    v.push(Blk::Coptic);
    v.push(Blk::CopticEpactNumbers);
    v.push(Blk::CountingRod);
    v.push(Blk::Cuneiform);
    v.push(Blk::CuneiformNumbers);
    v.push(Blk::CurrencySymbols);
    v.push(Blk::CypriotSyllabary);
    v.push(Blk::CyproMinoan);
    v.push(Blk::Cyrillic);
    v.push(Blk::CyrillicExtA);
    v.push(Blk::CyrillicExtB);
    v.push(Blk::CyrillicExtC);
    v.push(Blk::CyrillicExtD);
    v.push(Blk::CyrillicSup);
    v.push(Blk::Deseret);
    v.push(Blk::Devanagari);
    v.push(Blk::DevanagariExt);
    v.push(Blk::DevanagariExtA);
    v.push(Blk::Diacriticals);
    v.push(Blk::DiacriticalsExt);
    v.push(Blk::DiacriticalsForSymbols);
    v.push(Blk::DiacriticalsSup);
    v.push(Blk::Dingbats);
    v.push(Blk::DivesAkuru);
    v.push(Blk::Dogra);
    v.push(Blk::Domino);
    v.push(Blk::Duployan);
    v.push(Blk::EarlyDynasticCuneiform);
    v.push(Blk::EgyptianHieroglyphFormatControls);
    v.push(Blk::EgyptianHieroglyphs);
    v.push(Blk::Elbasan);
    v.push(Blk::Elymaic);
    v.push(Blk::Emoticons);
    v.push(Blk::EnclosedAlphanum);
    v.push(Blk::EnclosedAlphanumSup);
    v.push(Blk::EnclosedCjk);
    v.push(Blk::EnclosedIdeographicSup);
    v.push(Blk::Ethiopic);
    v.push(Blk::EthiopicExt);
    v.push(Blk::EthiopicExtA);
    v.push(Blk::EthiopicExtB);
    v.push(Blk::EthiopicSup);
    v.push(Blk::GeometricShapes);
    v.push(Blk::GeometricShapesExt);
    v.push(Blk::Georgian);
    v.push(Blk::GeorgianExt);
    v.push(Blk::GeorgianSup);
    v.push(Blk::Glagolitic);
    v.push(Blk::GlagoliticSup);
    v.push(Blk::Gothic);
    v.push(Blk::Grantha);
    v.push(Blk::Greek);
    v.push(Blk::GreekExt);
    v.push(Blk::Gujarati);
    v.push(Blk::GunjalaGondi);
    v.push(Blk::Gurmukhi);
    v.push(Blk::HalfAndFullForms);
    v.push(Blk::HalfMarks);
    v.push(Blk::Hangul);
    v.push(Blk::HanifiRohingya);
    v.push(Blk::Hanunoo);
    v.push(Blk::Hatran);
    v.push(Blk::Hebrew);
    v.push(Blk::HighPuSurrogates);
    v.push(Blk::HighSurrogates);
    v.push(Blk::Hiragana);
    v.push(Blk::Idc);
    v.push(Blk::IdeographicSymbols);
    v.push(Blk::ImperialAramaic);
    v.push(Blk::IndicNumberForms);
    v.push(Blk::IndicSiyaqNumbers);
    v.push(Blk::InscriptionalPahlavi);
    v.push(Blk::InscriptionalParthian);
    v.push(Blk::IpaExt);
    v.push(Blk::Jamo);
    v.push(Blk::JamoExtA);
    v.push(Blk::JamoExtB);
    v.push(Blk::Javanese);
    v.push(Blk::Kaithi);
    v.push(Blk::KaktovikNumerals);
    v.push(Blk::KanaExtA);
    v.push(Blk::KanaExtB);
    v.push(Blk::KanaSup);
    v.push(Blk::Kanbun);
    v.push(Blk::Kangxi);
    v.push(Blk::Kannada);
    v.push(Blk::Katakana);
    v.push(Blk::KatakanaExt);
    v.push(Blk::Kawi);
    v.push(Blk::KayahLi);
    v.push(Blk::Kharoshthi);
    v.push(Blk::KhitanSmallScript);
    v.push(Blk::Khmer);
    v.push(Blk::KhmerSymbols);
    v.push(Blk::Khojki);
    v.push(Blk::Khudawadi);
    v.push(Blk::Lao);
    v.push(Blk::Latin1Sup);
    v.push(Blk::LatinExtA);
    v.push(Blk::LatinExtAdditional);
    v.push(Blk::LatinExtB);
    v.push(Blk::LatinExtC);
    v.push(Blk::LatinExtD);
    v.push(Blk::LatinExtE);
    v.push(Blk::LatinExtF);
    v.push(Blk::LatinExtG);
    v.push(Blk::Lepcha);
    v.push(Blk::LetterlikeSymbols);
    v.push(Blk::Limbu);
    v.push(Blk::LinearA);
    v.push(Blk::LinearBIdeograms);
    v.push(Blk::LinearBSyllabary);
    v.push(Blk::Lisu);
    v.push(Blk::LisuSup);
    v.push(Blk::LowSurrogates);
    v.push(Blk::Lycian);
    v.push(Blk::Lydian);
    v.push(Blk::Mahajani);
    v.push(Blk::Mahjong);
    v.push(Blk::Makasar);
    v.push(Blk::Malayalam);
    v.push(Blk::Mandaic);
    v.push(Blk::Manichaean);
    v.push(Blk::Marchen);
    v.push(Blk::MasaramGondi);
    v.push(Blk::MathAlphanum);
    v.push(Blk::MathOperators);
    v.push(Blk::MayanNumerals);
    v.push(Blk::Medefaidrin);
    v.push(Blk::MeeteiMayek);
    v.push(Blk::MeeteiMayekExt);
    v.push(Blk::MendeKikakui);
    v.push(Blk::MeroiticCursive);
    v.push(Blk::MeroiticHieroglyphs);
    v.push(Blk::Miao);
    v.push(Blk::MiscArrows);
    v.push(Blk::MiscMathSymbolsA);
    v.push(Blk::MiscMathSymbolsB);
    v.push(Blk::MiscPictographs);
    v.push(Blk::MiscSymbols);
    v.push(Blk::MiscTechnical);
    v.push(Blk::Modi);
    v.push(Blk::ModifierLetters);
    v.push(Blk::ModifierToneLetters);
    v.push(Blk::Mongolian);
    v.push(Blk::MongolianSup);
    v.push(Blk::Mro);
    v.push(Blk::Multani);
    v.push(Blk::Music);
    v.push(Blk::Myanmar);
    v.push(Blk::MyanmarExtA);
    v.push(Blk::MyanmarExtB);
    v.push(Blk::Nabataean);
    v.push(Blk::NagMundari);
    v.push(Blk::Nandinagari);
    v.push(Blk::Nb);
    v.push(Blk::NewTaiLue);
    v.push(Blk::Newa);
    v.push(Blk::Nko);
    v.push(Blk::NumberForms);
    v.push(Blk::Nushu);
    v.push(Blk::NyiakengPuachueHmong);
    v.push(Blk::Ocr);
    v.push(Blk::Ogham);
    v.push(Blk::OlChiki);
    v.push(Blk::OldHungarian);
    v.push(Blk::OldItalic);
    v.push(Blk::OldNorthArabian);
    v.push(Blk::OldPermic);
    v.push(Blk::OldPersian);
    v.push(Blk::OldSogdian);
    v.push(Blk::OldSouthArabian);
    v.push(Blk::OldTurkic);
    v.push(Blk::OldUyghur);
    v.push(Blk::Oriya);
    v.push(Blk::OrnamentalDingbats);
    v.push(Blk::Osage);
    v.push(Blk::Osmanya);
    v.push(Blk::OttomanSiyaqNumbers);
    v.push(Blk::PahawhHmong);
    v.push(Blk::Palmyrene);
    v.push(Blk::PauCinHau);
    v.push(Blk::PhagsPa);
    v.push(Blk::Phaistos);
    v.push(Blk::Phoenician);
    v.push(Blk::PhoneticExt);
    v.push(Blk::PhoneticExtSup);
    v.push(Blk::PlayingCards);
    v.push(Blk::PsalterPahlavi);
    v.push(Blk::Pua);
    v.push(Blk::Punctuation);
    v.push(Blk::Rejang);
    v.push(Blk::Rumi);
    v.push(Blk::Runic);
    v.push(Blk::Samaritan);
    v.push(Blk::Saurashtra);
    v.push(Blk::Sharada);
    v.push(Blk::Shavian);
    v.push(Blk::ShorthandFormatControls);
    v.push(Blk::Siddham);
    v.push(Blk::Sinhala);
    v.push(Blk::SinhalaArchaicNumbers);
    v.push(Blk::SmallForms);
    v.push(Blk::SmallKanaExt);
    v.push(Blk::Sogdian);
    v.push(Blk::SoraSompeng);
    v.push(Blk::Soyombo);
    v.push(Blk::Specials);
    v.push(Blk::Sundanese);
    v.push(Blk::SundaneseSup);
    v.push(Blk::SupArrowsA);
    v.push(Blk::SupArrowsB);
    v.push(Blk::SupArrowsC);
    v.push(Blk::SupMathOperators);
    v.push(Blk::SupPuaA);
    v.push(Blk::SupPuaB);
    v.push(Blk::SupPunctuation);
    v.push(Blk::SupSymbolsAndPictographs);
    v.push(Blk::SuperAndSub);
    v.push(Blk::SuttonSignwriting);
    v.push(Blk::SylotiNagri);
    v.push(Blk::SymbolsAndPictographsExtA);
    v.push(Blk::SymbolsForLegacyComputing);
    v.push(Blk::Syriac);
    v.push(Blk::SyriacSup);
    v.push(Blk::Tagalog);
    v.push(Blk::Tagbanwa);
    v.push(Blk::Tags);
    v.push(Blk::TaiLe);
    v.push(Blk::TaiTham);
    v.push(Blk::TaiViet);
    v.push(Blk::TaiXuanJing);
    v.push(Blk::Takri);
    v.push(Blk::Tamil);
    v.push(Blk::TamilSup);
    v.push(Blk::Tangsa);
    v.push(Blk::Tangut);
    v.push(Blk::TangutComponents);
    v.push(Blk::TangutSup);
    v.push(Blk::Telugu);
    v.push(Blk::Thaana);
    v.push(Blk::Thai);
    v.push(Blk::Tibetan);
    v.push(Blk::Tifinagh);
    v.push(Blk::Tirhuta);
    v.push(Blk::Toto);
    v.push(Blk::TransportAndMap);
    v.push(Blk::Ucas);
    v.push(Blk::UcasExt);
    v.push(Blk::UcasExtA);
    v.push(Blk::Ugaritic);
    v.push(Blk::Vai);
    v.push(Blk::VedicExt);
    v.push(Blk::VerticalForms);
    v.push(Blk::Vithkuqi);
    v.push(Blk::Vs);
    v.push(Blk::VsSup);
    v.push(Blk::Wancho);
    v.push(Blk::WarangCiti);
    v.push(Blk::Yezidi);
    v.push(Blk::YiRadicals);
    v.push(Blk::YiSyllables);
    v.push(Blk::Yijing);
    v.push(Blk::ZanabazarSquare);
    v.push(Blk::ZnamennyMusic);

    let blocks: Vec<Value> = v.iter().map(|&x| -> Value {
        let mut begin = 0;
        let mut end = 0;

        for i in 0x0000u32..=0x10FFFFu32 {
            if CodePoint::new(i).unwrap().blk() == x {
                if begin == 0 {
                    begin = i;
                }
                end = i;
            }
        }

        json!({
            "property_name": {
                "full": x.property_value_name().full,
                "abbr": x.property_value_name().abbr,
            },
            "range_begin": CodePoint::new(begin).unwrap().to_string(),
            "range_end": CodePoint::new(end).unwrap().to_string(),
        })
    }).collect();

    json!({
        "unicode_version": UNICODE_VERSION.to_string(),
        "seshat_version": SESHAT_VERSION,
        "blocks": blocks,
    })
}

pub fn browse_blocks_block_api_v3(block: String) -> Value {
    let mut blk = Blk::Adlam;
    let mut code_points: Vec<String> = vec![];

    if block.as_str() == Blk::Adlam.property_value_name().abbr {
        blk = Blk::Adlam;
    } else if block.as_str() == Blk::AegeanNumbers.property_value_name().abbr {
        blk = Blk::AegeanNumbers;
    } else if block.as_str() == Blk::AegeanNumbers.property_value_name().abbr {
        blk = Blk::AegeanNumbers;
    } else if block.as_str() == Blk::Ahom.property_value_name().abbr {
        blk = Blk::Ahom;
    } else if block.as_str() == Blk::Alchemical.property_value_name().abbr {
        blk = Blk::Alchemical;
    } else if block.as_str() == Blk::AlphabeticPf.property_value_name().abbr {
        blk = Blk::AlphabeticPf;
    } else if block.as_str() == Blk::AnatolianHieroglyphs.property_value_name().abbr {
        blk = Blk::AnatolianHieroglyphs;
    } else if block.as_str() == Blk::AncientGreekMusic.property_value_name().abbr {
        blk = Blk::AncientGreekMusic;
    } else if block.as_str() == Blk::AncientGreekNumbers.property_value_name().abbr {
        blk = Blk::AncientGreekNumbers;
    } else if block.as_str() == Blk::AncientSymbols.property_value_name().abbr {
        blk = Blk::AncientSymbols;
    } else if block.as_str() == Blk::Arabic.property_value_name().abbr {
        blk = Blk::Arabic;
    } else if block.as_str() == Blk::ArabicExtA.property_value_name().abbr {
        blk = Blk::ArabicExtA;
    } else if block.as_str() == Blk::ArabicExtB.property_value_name().abbr {
        blk = Blk::ArabicExtB;
    } else if block.as_str() == Blk::ArabicExtC.property_value_name().abbr {
        blk = Blk::ArabicExtC;
    } else if block.as_str() == Blk::ArabicMath.property_value_name().abbr {
        blk = Blk::ArabicMath;
    } else if block.as_str() == Blk::ArabicPfA.property_value_name().abbr {
        blk = Blk::ArabicPfA;
    } else if block.as_str() == Blk::ArabicPfB.property_value_name().abbr {
        blk = Blk::ArabicPfB;
    } else if block.as_str() == Blk::ArabicSup.property_value_name().abbr {
        blk = Blk::ArabicSup;
    } else if block.as_str() == Blk::Armenian.property_value_name().abbr {
        blk = Blk::Armenian;
    } else if block.as_str() == Blk::Arrows.property_value_name().abbr {
        blk = Blk::Arrows;
    } else if block.as_str() == Blk::Ascii.property_value_name().abbr {
        blk = Blk::Ascii;
    } else if block.as_str() == Blk::Avestan.property_value_name().abbr {
        blk = Blk::Avestan;
    } else if block.as_str() == Blk::Balinese.property_value_name().abbr {
        blk = Blk::Balinese;
    } else if block.as_str() == Blk::Bamum.property_value_name().abbr {
        blk = Blk::Bamum;
    } else if block.as_str() == Blk::BamumSup.property_value_name().abbr {
        blk = Blk::BamumSup;
    } else if block.as_str() == Blk::BassaVah.property_value_name().abbr {
        blk = Blk::BassaVah;
    } else if block.as_str() == Blk::Batak.property_value_name().abbr {
        blk = Blk::Batak;
    } else if block.as_str() == Blk::Bengali.property_value_name().abbr {
        blk = Blk::Bengali;
    } else if block.as_str() == Blk::Bhaiksuki.property_value_name().abbr {
        blk = Blk::Bhaiksuki;
    } else if block.as_str() == Blk::BlockElements.property_value_name().abbr {
        blk = Blk::BlockElements;
    } else if block.as_str() == Blk::Bopomofo.property_value_name().abbr {
        blk = Blk::Bopomofo;
    } else if block.as_str() == Blk::BopomofoExt.property_value_name().abbr {
        blk = Blk::BopomofoExt;
    } else if block.as_str() == Blk::BoxDrawing.property_value_name().abbr {
        blk = Blk::BoxDrawing;
    } else if block.as_str() == Blk::Brahmi.property_value_name().abbr {
        blk = Blk::Brahmi;
    } else if block.as_str() == Blk::Braille.property_value_name().abbr {
        blk = Blk::Braille;
    } else if block.as_str() == Blk::Buginese.property_value_name().abbr {
        blk = Blk::Buginese;
    } else if block.as_str() == Blk::Buhid.property_value_name().abbr {
        blk = Blk::Buhid;
    } else if block.as_str() == Blk::ByzantineMusic.property_value_name().abbr {
        blk = Blk::ByzantineMusic;
    } else if block.as_str() == Blk::Carian.property_value_name().abbr {
        blk = Blk::Carian;
    } else if block.as_str() == Blk::CaucasianAlbanian.property_value_name().abbr {
        blk = Blk::CaucasianAlbanian;
    } else if block.as_str() == Blk::Chakma.property_value_name().abbr {
        blk = Blk::Chakma;
    } else if block.as_str() == Blk::Cham.property_value_name().abbr {
        blk = Blk::Cham;
    } else if block.as_str() == Blk::Cherokee.property_value_name().abbr {
        blk = Blk::Cherokee;
    } else if block.as_str() == Blk::CherokeeSup.property_value_name().abbr {
        blk = Blk::CherokeeSup;
    } else if block.as_str() == Blk::ChessSymbols.property_value_name().abbr {
        blk = Blk::ChessSymbols;
    } else if block.as_str() == Blk::Chorasmian.property_value_name().abbr {
        blk = Blk::Chorasmian;
    } else if block.as_str() == Blk::Cjk.property_value_name().abbr {
        blk = Blk::Cjk;
    } else if block.as_str() == Blk::CjkCompat.property_value_name().abbr {
        blk = Blk::CjkCompat;
    } else if block.as_str() == Blk::CjkCompatForms.property_value_name().abbr {
        blk = Blk::CjkCompatForms;
    } else if block.as_str() == Blk::CjkCompatIdeographs.property_value_name().abbr {
        blk = Blk::CjkCompatIdeographs;
    } else if block.as_str() == Blk::CjkCompatIdeographsSup.property_value_name().abbr {
        blk = Blk::CjkCompatIdeographsSup;
    } else if block.as_str() == Blk::CjkExtA.property_value_name().abbr {
        blk = Blk::CjkExtA;
    } else if block.as_str() == Blk::CjkExtB.property_value_name().abbr {
        blk = Blk::CjkExtB;
    } else if block.as_str() == Blk::CjkExtC.property_value_name().abbr {
        blk = Blk::CjkExtC;
    } else if block.as_str() == Blk::CjkExtD.property_value_name().abbr {
        blk = Blk::CjkExtD;
    } else if block.as_str() == Blk::CjkExtE.property_value_name().abbr {
        blk = Blk::CjkExtE;
    } else if block.as_str() == Blk::CjkExtF.property_value_name().abbr {
        blk = Blk::CjkExtF;
    } else if block.as_str() == Blk::CjkExtG.property_value_name().abbr {
        blk = Blk::CjkExtG;
    } else if block.as_str() == Blk::CjkExtH.property_value_name().abbr {
        blk = Blk::CjkExtH;
    } else if block.as_str() == Blk::CjkExtI.property_value_name().abbr {
        blk = Blk::CjkExtI;
    } else if block.as_str() == Blk::CjkRadicalsSup.property_value_name().abbr {
        blk = Blk::CjkRadicalsSup;
    } else if block.as_str() == Blk::CjkStrokes.property_value_name().abbr {
        blk = Blk::CjkStrokes;
    } else if block.as_str() == Blk::CjkSymbols.property_value_name().abbr {
        blk = Blk::CjkSymbols;
    } else if block.as_str() == Blk::CompatJamo.property_value_name().abbr {
        blk = Blk::CompatJamo;
    } else if block.as_str() == Blk::ControlPictures.property_value_name().abbr {
        blk = Blk::ControlPictures;
    } else if block.as_str() == Blk::Coptic.property_value_name().abbr {
        blk = Blk::Coptic;
    } else if block.as_str() == Blk::CopticEpactNumbers.property_value_name().abbr {
        blk = Blk::CopticEpactNumbers;
    } else if block.as_str() == Blk::CountingRod.property_value_name().abbr {
        blk = Blk::CountingRod;
    } else if block.as_str() == Blk::Cuneiform.property_value_name().abbr {
        blk = Blk::Cuneiform;
    } else if block.as_str() == Blk::CuneiformNumbers.property_value_name().abbr {
        blk = Blk::CuneiformNumbers;
    } else if block.as_str() == Blk::CurrencySymbols.property_value_name().abbr {
        blk = Blk::CurrencySymbols;
    } else if block.as_str() == Blk::CypriotSyllabary.property_value_name().abbr {
        blk = Blk::CypriotSyllabary;
    } else if block.as_str() == Blk::CyproMinoan.property_value_name().abbr {
        blk = Blk::CyproMinoan;
    } else if block.as_str() == Blk::Cyrillic.property_value_name().abbr {
        blk = Blk::Cyrillic;
    } else if block.as_str() == Blk::CyrillicExtA.property_value_name().abbr {
        blk = Blk::CyrillicExtA;
    } else if block.as_str() == Blk::CyrillicExtB.property_value_name().abbr {
        blk = Blk::CyrillicExtB;
    } else if block.as_str() == Blk::CyrillicExtC.property_value_name().abbr {
        blk = Blk::CyrillicExtC;
    } else if block.as_str() == Blk::CyrillicExtD.property_value_name().abbr {
        blk = Blk::CyrillicExtD;
    } else if block.as_str() == Blk::CyrillicSup.property_value_name().abbr {
        blk = Blk::CyrillicSup;
    } else if block.as_str() == Blk::Deseret.property_value_name().abbr {
        blk = Blk::Deseret;
    } else if block.as_str() == Blk::Devanagari.property_value_name().abbr {
        blk = Blk::Devanagari;
    } else if block.as_str() == Blk::DevanagariExt.property_value_name().abbr {
        blk = Blk::DevanagariExt;
    } else if block.as_str() == Blk::DevanagariExtA.property_value_name().abbr {
        blk = Blk::DevanagariExtA;
    } else if block.as_str() == Blk::Diacriticals.property_value_name().abbr {
        blk = Blk::Diacriticals;
    } else if block.as_str() == Blk::DiacriticalsExt.property_value_name().abbr {
        blk = Blk::DiacriticalsExt;
    } else if block.as_str() == Blk::DiacriticalsForSymbols.property_value_name().abbr {
        blk = Blk::DiacriticalsForSymbols;
    } else if block.as_str() == Blk::DiacriticalsSup.property_value_name().abbr {
        blk = Blk::DiacriticalsSup;
    } else if block.as_str() == Blk::Dingbats.property_value_name().abbr {
        blk = Blk::Dingbats;
    } else if block.as_str() == Blk::DivesAkuru.property_value_name().abbr {
        blk = Blk::DivesAkuru;
    } else if block.as_str() == Blk::Dogra.property_value_name().abbr {
        blk = Blk::Dogra;
    } else if block.as_str() == Blk::Domino.property_value_name().abbr {
        blk = Blk::Domino;
    } else if block.as_str() == Blk::Duployan.property_value_name().abbr {
        blk = Blk::Duployan;
    } else if block.as_str() == Blk::EarlyDynasticCuneiform.property_value_name().abbr {
        blk = Blk::EarlyDynasticCuneiform;
    } else if block.as_str() == Blk::EgyptianHieroglyphFormatControls.property_value_name().abbr {
        blk = Blk::EgyptianHieroglyphFormatControls;
    } else if block.as_str() == Blk::EgyptianHieroglyphs.property_value_name().abbr {
        blk = Blk::EgyptianHieroglyphs;
    } else if block.as_str() == Blk::Elbasan.property_value_name().abbr {
        blk = Blk::Elbasan;
    } else if block.as_str() == Blk::Elymaic.property_value_name().abbr {
        blk = Blk::Elymaic;
    } else if block.as_str() == Blk::Emoticons.property_value_name().abbr {
        blk = Blk::Emoticons;
    } else if block.as_str() == Blk::EnclosedAlphanum.property_value_name().abbr {
        blk = Blk::EnclosedAlphanum;
    } else if block.as_str() == Blk::EnclosedAlphanumSup.property_value_name().abbr {
        blk = Blk::EnclosedAlphanumSup;
    } else if block.as_str() == Blk::EnclosedCjk.property_value_name().abbr {
        blk = Blk::EnclosedCjk;
    } else if block.as_str() == Blk::EnclosedIdeographicSup.property_value_name().abbr {
        blk = Blk::EnclosedIdeographicSup;
    } else if block.as_str() == Blk::Ethiopic.property_value_name().abbr {
        blk = Blk::Ethiopic;
    } else if block.as_str() == Blk::EthiopicExt.property_value_name().abbr {
        blk = Blk::EthiopicExt;
    } else if block.as_str() == Blk::EthiopicExtA.property_value_name().abbr {
        blk = Blk::EthiopicExtA;
    } else if block.as_str() == Blk::EthiopicExtB.property_value_name().abbr {
        blk = Blk::EthiopicExtB;
    } else if block.as_str() == Blk::EthiopicSup.property_value_name().abbr {
        blk = Blk::EthiopicSup;
    } else if block.as_str() == Blk::GeometricShapes.property_value_name().abbr {
        blk = Blk::GeometricShapes;
    } else if block.as_str() == Blk::GeometricShapesExt.property_value_name().abbr {
        blk = Blk::GeometricShapesExt;
    } else if block.as_str() == Blk::Georgian.property_value_name().abbr {
        blk = Blk::Georgian;
    } else if block.as_str() == Blk::GeorgianExt.property_value_name().abbr {
        blk = Blk::GeorgianExt;
    } else if block.as_str() == Blk::GeorgianSup.property_value_name().abbr {
        blk = Blk::GeorgianSup;
    } else if block.as_str() == Blk::Glagolitic.property_value_name().abbr {
        blk = Blk::Glagolitic;
    } else if block.as_str() == Blk::GlagoliticSup.property_value_name().abbr {
        blk = Blk::GlagoliticSup;
    } else if block.as_str() == Blk::Gothic.property_value_name().abbr {
        blk = Blk::Gothic;
    } else if block.as_str() == Blk::Grantha.property_value_name().abbr {
        blk = Blk::Grantha;
    } else if block.as_str() == Blk::Greek.property_value_name().abbr {
        blk = Blk::Greek;
    } else if block.as_str() == Blk::GreekExt.property_value_name().abbr {
        blk = Blk::GreekExt;
    } else if block.as_str() == Blk::Gujarati.property_value_name().abbr {
        blk = Blk::Gujarati;
    } else if block.as_str() == Blk::GunjalaGondi.property_value_name().abbr {
        blk = Blk::GunjalaGondi;
    } else if block.as_str() == Blk::Gurmukhi.property_value_name().abbr {
        blk = Blk::Gurmukhi;
    } else if block.as_str() == Blk::HalfAndFullForms.property_value_name().abbr {
        blk = Blk::HalfAndFullForms;
    } else if block.as_str() == Blk::HalfMarks.property_value_name().abbr {
        blk = Blk::HalfMarks;
    } else if block.as_str() == Blk::Hangul.property_value_name().abbr {
        blk = Blk::Hangul;
    } else if block.as_str() == Blk::HanifiRohingya.property_value_name().abbr {
        blk = Blk::HanifiRohingya;
    } else if block.as_str() == Blk::Hanunoo.property_value_name().abbr {
        blk = Blk::Hanunoo;
    } else if block.as_str() == Blk::Hatran.property_value_name().abbr {
        blk = Blk::Hatran;
    } else if block.as_str() == Blk::Hebrew.property_value_name().abbr {
        blk = Blk::Hebrew;
    } else if block.as_str() == Blk::HighPuSurrogates.property_value_name().abbr {
        blk = Blk::HighPuSurrogates;
    } else if block.as_str() == Blk::HighSurrogates.property_value_name().abbr {
        blk = Blk::HighSurrogates;
    } else if block.as_str() == Blk::Hiragana.property_value_name().abbr {
        blk = Blk::Hiragana;
    } else if block.as_str() == Blk::Idc.property_value_name().abbr {
        blk = Blk::Idc;
    } else if block.as_str() == Blk::IdeographicSymbols.property_value_name().abbr {
        blk = Blk::IdeographicSymbols;
    } else if block.as_str() == Blk::ImperialAramaic.property_value_name().abbr {
        blk = Blk::ImperialAramaic;
    } else if block.as_str() == Blk::IndicNumberForms.property_value_name().abbr {
        blk = Blk::IndicNumberForms;
    } else if block.as_str() == Blk::IndicSiyaqNumbers.property_value_name().abbr {
        blk = Blk::IndicSiyaqNumbers;
    } else if block.as_str() == Blk::InscriptionalPahlavi.property_value_name().abbr {
        blk = Blk::InscriptionalPahlavi;
    } else if block.as_str() == Blk::InscriptionalParthian.property_value_name().abbr {
        blk = Blk::InscriptionalParthian;
    } else if block.as_str() == Blk::IpaExt.property_value_name().abbr {
        blk = Blk::IpaExt;
    } else if block.as_str() == Blk::Jamo.property_value_name().abbr {
        blk = Blk::Jamo;
    } else if block.as_str() == Blk::JamoExtA.property_value_name().abbr {
        blk = Blk::JamoExtA;
    } else if block.as_str() == Blk::JamoExtB.property_value_name().abbr {
        blk = Blk::JamoExtB;
    } else if block.as_str() == Blk::Javanese.property_value_name().abbr {
        blk = Blk::Javanese;
    } else if block.as_str() == Blk::Kaithi.property_value_name().abbr {
        blk = Blk::Kaithi;
    } else if block.as_str() == Blk::KaktovikNumerals.property_value_name().abbr {
        blk = Blk::KaktovikNumerals;
    } else if block.as_str() == Blk::KanaExtA.property_value_name().abbr {
        blk = Blk::KanaExtA;
    } else if block.as_str() == Blk::KanaExtB.property_value_name().abbr {
        blk = Blk::KanaExtB;
    } else if block.as_str() == Blk::KanaSup.property_value_name().abbr {
        blk = Blk::KanaSup;
    } else if block.as_str() == Blk::Kanbun.property_value_name().abbr {
        blk = Blk::Kanbun;
    } else if block.as_str() == Blk::Kangxi.property_value_name().abbr {
        blk = Blk::Kangxi;
    } else if block.as_str() == Blk::Kannada.property_value_name().abbr {
        blk = Blk::Kannada;
    } else if block.as_str() == Blk::Katakana.property_value_name().abbr {
        blk = Blk::Katakana;
    } else if block.as_str() == Blk::KatakanaExt.property_value_name().abbr {
        blk = Blk::KatakanaExt;
    } else if block.as_str() == Blk::Kawi.property_value_name().abbr {
        blk = Blk::Kawi;
    } else if block.as_str() == Blk::KayahLi.property_value_name().abbr {
        blk = Blk::KayahLi;
    } else if block.as_str() == Blk::Kharoshthi.property_value_name().abbr {
        blk = Blk::Kharoshthi;
    } else if block.as_str() == Blk::KhitanSmallScript.property_value_name().abbr {
        blk = Blk::KhitanSmallScript;
    } else if block.as_str() == Blk::Khmer.property_value_name().abbr {
        blk = Blk::Khmer;
    } else if block.as_str() == Blk::KhmerSymbols.property_value_name().abbr {
        blk = Blk::KhmerSymbols;
    } else if block.as_str() == Blk::Khojki.property_value_name().abbr {
        blk = Blk::Khojki;
    } else if block.as_str() == Blk::Khudawadi.property_value_name().abbr {
        blk = Blk::Khudawadi;
    } else if block.as_str() == Blk::Lao.property_value_name().abbr {
        blk = Blk::Lao;
    } else if block.as_str() == Blk::Latin1Sup.property_value_name().abbr {
        blk = Blk::Latin1Sup;
    } else if block.as_str() == Blk::LatinExtA.property_value_name().abbr {
        blk = Blk::LatinExtA;
    } else if block.as_str() == Blk::LatinExtAdditional.property_value_name().abbr {
        blk = Blk::LatinExtAdditional;
    } else if block.as_str() == Blk::LatinExtB.property_value_name().abbr {
        blk = Blk::LatinExtB;
    } else if block.as_str() == Blk::LatinExtC.property_value_name().abbr {
        blk = Blk::LatinExtC;
    } else if block.as_str() == Blk::LatinExtD.property_value_name().abbr {
        blk = Blk::LatinExtD;
    } else if block.as_str() == Blk::LatinExtE.property_value_name().abbr {
        blk = Blk::LatinExtE;
    } else if block.as_str() == Blk::LatinExtF.property_value_name().abbr {
        blk = Blk::LatinExtF;
    } else if block.as_str() == Blk::LatinExtG.property_value_name().abbr {
        blk = Blk::LatinExtG;
    } else if block.as_str() == Blk::Lepcha.property_value_name().abbr {
        blk = Blk::Lepcha;
    } else if block.as_str() == Blk::LetterlikeSymbols.property_value_name().abbr {
        blk = Blk::LetterlikeSymbols;
    } else if block.as_str() == Blk::Limbu.property_value_name().abbr {
        blk = Blk::Limbu;
    } else if block.as_str() == Blk::LinearA.property_value_name().abbr {
        blk = Blk::LinearA;
    } else if block.as_str() == Blk::LinearBIdeograms.property_value_name().abbr {
        blk = Blk::LinearBIdeograms;
    } else if block.as_str() == Blk::LinearBSyllabary.property_value_name().abbr {
        blk = Blk::LinearBSyllabary;
    } else if block.as_str() == Blk::Lisu.property_value_name().abbr {
        blk = Blk::Lisu;
    } else if block.as_str() == Blk::LisuSup.property_value_name().abbr {
        blk = Blk::LisuSup;
    } else if block.as_str() == Blk::LowSurrogates.property_value_name().abbr {
        blk = Blk::LowSurrogates;
    } else if block.as_str() == Blk::Lycian.property_value_name().abbr {
        blk = Blk::Lycian;
    } else if block.as_str() == Blk::Lydian.property_value_name().abbr {
        blk = Blk::Lydian;
    } else if block.as_str() == Blk::Mahajani.property_value_name().abbr {
        blk = Blk::Mahajani;
    } else if block.as_str() == Blk::Mahjong.property_value_name().abbr {
        blk = Blk::Mahjong;
    } else if block.as_str() == Blk::Makasar.property_value_name().abbr {
        blk = Blk::Makasar;
    } else if block.as_str() == Blk::Malayalam.property_value_name().abbr {
        blk = Blk::Malayalam;
    } else if block.as_str() == Blk::Mandaic.property_value_name().abbr {
        blk = Blk::Mandaic;
    } else if block.as_str() == Blk::Manichaean.property_value_name().abbr {
        blk = Blk::Manichaean;
    } else if block.as_str() == Blk::Marchen.property_value_name().abbr {
        blk = Blk::Marchen;
    } else if block.as_str() == Blk::MasaramGondi.property_value_name().abbr {
        blk = Blk::MasaramGondi;
    } else if block.as_str() == Blk::MathAlphanum.property_value_name().abbr {
        blk = Blk::MathAlphanum;
    } else if block.as_str() == Blk::MathOperators.property_value_name().abbr {
        blk = Blk::MathOperators;
    } else if block.as_str() == Blk::MayanNumerals.property_value_name().abbr {
        blk = Blk::MayanNumerals;
    } else if block.as_str() == Blk::Medefaidrin.property_value_name().abbr {
        blk = Blk::Medefaidrin;
    } else if block.as_str() == Blk::MeeteiMayek.property_value_name().abbr {
        blk = Blk::MeeteiMayek;
    } else if block.as_str() == Blk::MeeteiMayekExt.property_value_name().abbr {
        blk = Blk::MeeteiMayekExt;
    } else if block.as_str() == Blk::MendeKikakui.property_value_name().abbr {
        blk = Blk::MendeKikakui;
    } else if block.as_str() == Blk::MeroiticCursive.property_value_name().abbr {
        blk = Blk::MeroiticCursive;
    } else if block.as_str() == Blk::MeroiticHieroglyphs.property_value_name().abbr {
        blk = Blk::MeroiticHieroglyphs;
    } else if block.as_str() == Blk::Miao.property_value_name().abbr {
        blk = Blk::Miao;
    } else if block.as_str() == Blk::MiscArrows.property_value_name().abbr {
        blk = Blk::MiscArrows;
    } else if block.as_str() == Blk::MiscMathSymbolsA.property_value_name().abbr {
        blk = Blk::MiscMathSymbolsA;
    } else if block.as_str() == Blk::MiscMathSymbolsB.property_value_name().abbr {
        blk = Blk::MiscMathSymbolsB;
    } else if block.as_str() == Blk::MiscPictographs.property_value_name().abbr {
        blk = Blk::MiscPictographs;
    } else if block.as_str() == Blk::MiscSymbols.property_value_name().abbr {
        blk = Blk::MiscSymbols;
    } else if block.as_str() == Blk::MiscTechnical.property_value_name().abbr {
        blk = Blk::MiscTechnical;
    } else if block.as_str() == Blk::Modi.property_value_name().abbr {
        blk = Blk::Modi;
    } else if block.as_str() == Blk::ModifierLetters.property_value_name().abbr {
        blk = Blk::ModifierLetters;
    } else if block.as_str() == Blk::ModifierToneLetters.property_value_name().abbr {
        blk = Blk::ModifierToneLetters;
    } else if block.as_str() == Blk::Mongolian.property_value_name().abbr {
        blk = Blk::Mongolian;
    } else if block.as_str() == Blk::MongolianSup.property_value_name().abbr {
        blk = Blk::MongolianSup;
    } else if block.as_str() == Blk::Mro.property_value_name().abbr {
        blk = Blk::Mro;
    } else if block.as_str() == Blk::Multani.property_value_name().abbr {
        blk = Blk::Multani;
    } else if block.as_str() == Blk::Music.property_value_name().abbr {
        blk = Blk::Music;
    } else if block.as_str() == Blk::Myanmar.property_value_name().abbr {
        blk = Blk::Myanmar;
    } else if block.as_str() == Blk::MyanmarExtA.property_value_name().abbr {
        blk = Blk::MyanmarExtA;
    } else if block.as_str() == Blk::MyanmarExtB.property_value_name().abbr {
        blk = Blk::MyanmarExtB;
    } else if block.as_str() == Blk::Nabataean.property_value_name().abbr {
        blk = Blk::Nabataean;
    } else if block.as_str() == Blk::NagMundari.property_value_name().abbr {
        blk = Blk::NagMundari;
    } else if block.as_str() == Blk::Nandinagari.property_value_name().abbr {
        blk = Blk::Nandinagari;
    } else if block.as_str() == Blk::Nb.property_value_name().abbr {
        blk = Blk::Nb;
    } else if block.as_str() == Blk::NewTaiLue.property_value_name().abbr {
        blk = Blk::NewTaiLue;
    } else if block.as_str() == Blk::Newa.property_value_name().abbr {
        blk = Blk::Newa;
    } else if block.as_str() == Blk::Nko.property_value_name().abbr {
        blk = Blk::Nko;
    } else if block.as_str() == Blk::NumberForms.property_value_name().abbr {
        blk = Blk::NumberForms;
    } else if block.as_str() == Blk::Nushu.property_value_name().abbr {
        blk = Blk::Nushu;
    } else if block.as_str() == Blk::NyiakengPuachueHmong.property_value_name().abbr {
        blk = Blk::NyiakengPuachueHmong;
    } else if block.as_str() == Blk::Ocr.property_value_name().abbr {
        blk = Blk::Ocr;
    } else if block.as_str() == Blk::Ogham.property_value_name().abbr {
        blk = Blk::Ogham;
    } else if block.as_str() == Blk::OlChiki.property_value_name().abbr {
        blk = Blk::OlChiki;
    } else if block.as_str() == Blk::OldHungarian.property_value_name().abbr {
        blk = Blk::OldHungarian;
    } else if block.as_str() == Blk::OldItalic.property_value_name().abbr {
        blk = Blk::OldItalic;
    } else if block.as_str() == Blk::OldNorthArabian.property_value_name().abbr {
        blk = Blk::OldNorthArabian;
    } else if block.as_str() == Blk::OldPermic.property_value_name().abbr {
        blk = Blk::OldPermic;
    } else if block.as_str() == Blk::OldPersian.property_value_name().abbr {
        blk = Blk::OldPersian;
    } else if block.as_str() == Blk::OldSogdian.property_value_name().abbr {
        blk = Blk::OldSogdian;
    } else if block.as_str() == Blk::OldSouthArabian.property_value_name().abbr {
        blk = Blk::OldSouthArabian;
    } else if block.as_str() == Blk::OldTurkic.property_value_name().abbr {
        blk = Blk::OldTurkic;
    } else if block.as_str() == Blk::OldUyghur.property_value_name().abbr {
        blk = Blk::OldUyghur;
    } else if block.as_str() == Blk::Oriya.property_value_name().abbr {
        blk = Blk::Oriya;
    } else if block.as_str() == Blk::OrnamentalDingbats.property_value_name().abbr {
        blk = Blk::OrnamentalDingbats;
    } else if block.as_str() == Blk::Osage.property_value_name().abbr {
        blk = Blk::Osage;
    } else if block.as_str() == Blk::Osmanya.property_value_name().abbr {
        blk = Blk::Osmanya;
    } else if block.as_str() == Blk::OttomanSiyaqNumbers.property_value_name().abbr {
        blk = Blk::OttomanSiyaqNumbers;
    } else if block.as_str() == Blk::PahawhHmong.property_value_name().abbr {
        blk = Blk::PahawhHmong;
    } else if block.as_str() == Blk::Palmyrene.property_value_name().abbr {
        blk = Blk::Palmyrene;
    } else if block.as_str() == Blk::PauCinHau.property_value_name().abbr {
        blk = Blk::PauCinHau;
    } else if block.as_str() == Blk::PhagsPa.property_value_name().abbr {
        blk = Blk::PhagsPa;
    } else if block.as_str() == Blk::Phaistos.property_value_name().abbr {
        blk = Blk::Phaistos;
    } else if block.as_str() == Blk::Phoenician.property_value_name().abbr {
        blk = Blk::Phoenician;
    } else if block.as_str() == Blk::PhoneticExt.property_value_name().abbr {
        blk = Blk::PhoneticExt;
    } else if block.as_str() == Blk::PhoneticExtSup.property_value_name().abbr {
        blk = Blk::PhoneticExtSup;
    } else if block.as_str() == Blk::PlayingCards.property_value_name().abbr {
        blk = Blk::PlayingCards;
    } else if block.as_str() == Blk::PsalterPahlavi.property_value_name().abbr {
        blk = Blk::PsalterPahlavi;
    } else if block.as_str() == Blk::Pua.property_value_name().abbr {
        blk = Blk::Pua;
    } else if block.as_str() == Blk::Punctuation.property_value_name().abbr {
        blk = Blk::Punctuation;
    } else if block.as_str() == Blk::Rejang.property_value_name().abbr {
        blk = Blk::Rejang;
    } else if block.as_str() == Blk::Rumi.property_value_name().abbr {
        blk = Blk::Rumi;
    } else if block.as_str() == Blk::Runic.property_value_name().abbr {
        blk = Blk::Runic;
    } else if block.as_str() == Blk::Samaritan.property_value_name().abbr {
        blk = Blk::Samaritan;
    } else if block.as_str() == Blk::Saurashtra.property_value_name().abbr {
        blk = Blk::Saurashtra;
    } else if block.as_str() == Blk::Sharada.property_value_name().abbr {
        blk = Blk::Sharada;
    } else if block.as_str() == Blk::Shavian.property_value_name().abbr {
        blk = Blk::Shavian;
    } else if block.as_str() == Blk::ShorthandFormatControls.property_value_name().abbr {
        blk = Blk::ShorthandFormatControls;
    } else if block.as_str() == Blk::Siddham.property_value_name().abbr {
        blk = Blk::Siddham;
    } else if block.as_str() == Blk::Sinhala.property_value_name().abbr {
        blk = Blk::Sinhala;
    } else if block.as_str() == Blk::SinhalaArchaicNumbers.property_value_name().abbr {
        blk = Blk::SinhalaArchaicNumbers;
    } else if block.as_str() == Blk::SmallForms.property_value_name().abbr {
        blk = Blk::SmallForms;
    } else if block.as_str() == Blk::SmallKanaExt.property_value_name().abbr {
        blk = Blk::SmallKanaExt;
    } else if block.as_str() == Blk::Sogdian.property_value_name().abbr {
        blk = Blk::Sogdian;
    } else if block.as_str() == Blk::SoraSompeng.property_value_name().abbr {
        blk = Blk::SoraSompeng;
    } else if block.as_str() == Blk::Soyombo.property_value_name().abbr {
        blk = Blk::Soyombo;
    } else if block.as_str() == Blk::Specials.property_value_name().abbr {
        blk = Blk::Specials;
    } else if block.as_str() == Blk::Sundanese.property_value_name().abbr {
        blk = Blk::Sundanese;
    } else if block.as_str() == Blk::SundaneseSup.property_value_name().abbr {
        blk = Blk::SundaneseSup;
    } else if block.as_str() == Blk::SupArrowsA.property_value_name().abbr {
        blk = Blk::SupArrowsA;
    } else if block.as_str() == Blk::SupArrowsB.property_value_name().abbr {
        blk = Blk::SupArrowsB;
    } else if block.as_str() == Blk::SupArrowsC.property_value_name().abbr {
        blk = Blk::SupArrowsC;
    } else if block.as_str() == Blk::SupMathOperators.property_value_name().abbr {
        blk = Blk::SupMathOperators;
    } else if block.as_str() == Blk::SupPuaA.property_value_name().abbr {
        blk = Blk::SupPuaA;
    } else if block.as_str() == Blk::SupPuaB.property_value_name().abbr {
        blk = Blk::SupPuaB;
    } else if block.as_str() == Blk::SupPunctuation.property_value_name().abbr {
        blk = Blk::SupPunctuation;
    } else if block.as_str() == Blk::SupSymbolsAndPictographs.property_value_name().abbr {
        blk = Blk::SupSymbolsAndPictographs;
    } else if block.as_str() == Blk::SuperAndSub.property_value_name().abbr {
        blk = Blk::SuperAndSub;
    } else if block.as_str() == Blk::SuttonSignwriting.property_value_name().abbr {
        blk = Blk::SuttonSignwriting;
    } else if block.as_str() == Blk::SylotiNagri.property_value_name().abbr {
        blk = Blk::SylotiNagri;
    } else if block.as_str() == Blk::SymbolsAndPictographsExtA.property_value_name().abbr {
        blk = Blk::SymbolsAndPictographsExtA;
    } else if block.as_str() == Blk::SymbolsForLegacyComputing.property_value_name().abbr {
        blk = Blk::SymbolsForLegacyComputing;
    } else if block.as_str() == Blk::Syriac.property_value_name().abbr {
        blk = Blk::Syriac;
    } else if block.as_str() == Blk::SyriacSup.property_value_name().abbr {
        blk = Blk::SyriacSup;
    } else if block.as_str() == Blk::Tagalog.property_value_name().abbr {
        blk = Blk::Tagalog;
    } else if block.as_str() == Blk::Tagbanwa.property_value_name().abbr {
        blk = Blk::Tagbanwa;
    } else if block.as_str() == Blk::Tags.property_value_name().abbr {
        blk = Blk::Tags;
    } else if block.as_str() == Blk::TaiLe.property_value_name().abbr {
        blk = Blk::TaiLe;
    } else if block.as_str() == Blk::TaiTham.property_value_name().abbr {
        blk = Blk::TaiTham;
    } else if block.as_str() == Blk::TaiViet.property_value_name().abbr {
        blk = Blk::TaiViet;
    } else if block.as_str() == Blk::TaiXuanJing.property_value_name().abbr {
        blk = Blk::TaiXuanJing;
    } else if block.as_str() == Blk::Takri.property_value_name().abbr {
        blk = Blk::Takri;
    } else if block.as_str() == Blk::Tamil.property_value_name().abbr {
        blk = Blk::Tamil;
    } else if block.as_str() == Blk::TamilSup.property_value_name().abbr {
        blk = Blk::TamilSup;
    } else if block.as_str() == Blk::Tangsa.property_value_name().abbr {
        blk = Blk::Tangsa;
    } else if block.as_str() == Blk::Tangut.property_value_name().abbr {
        blk = Blk::Tangut;
    } else if block.as_str() == Blk::TangutComponents.property_value_name().abbr {
        blk = Blk::TangutComponents;
    } else if block.as_str() == Blk::TangutSup.property_value_name().abbr {
        blk = Blk::TangutSup;
    } else if block.as_str() == Blk::Telugu.property_value_name().abbr {
        blk = Blk::Telugu;
    } else if block.as_str() == Blk::Thaana.property_value_name().abbr {
        blk = Blk::Thaana;
    } else if block.as_str() == Blk::Thai.property_value_name().abbr {
        blk = Blk::Thai;
    } else if block.as_str() == Blk::Tibetan.property_value_name().abbr {
        blk = Blk::Tibetan;
    } else if block.as_str() == Blk::Tifinagh.property_value_name().abbr {
        blk = Blk::Tifinagh;
    } else if block.as_str() == Blk::Tirhuta.property_value_name().abbr {
        blk = Blk::Tirhuta;
    } else if block.as_str() == Blk::Toto.property_value_name().abbr {
        blk = Blk::Toto;
    } else if block.as_str() == Blk::TransportAndMap.property_value_name().abbr {
        blk = Blk::TransportAndMap;
    } else if block.as_str() == Blk::Ucas.property_value_name().abbr {
        blk = Blk::Ucas;
    } else if block.as_str() == Blk::UcasExt.property_value_name().abbr {
        blk = Blk::UcasExt;
    } else if block.as_str() == Blk::UcasExtA.property_value_name().abbr {
        blk = Blk::UcasExtA;
    } else if block.as_str() == Blk::Ugaritic.property_value_name().abbr {
        blk = Blk::Ugaritic;
    } else if block.as_str() == Blk::Vai.property_value_name().abbr {
        blk = Blk::Vai;
    } else if block.as_str() == Blk::VedicExt.property_value_name().abbr {
        blk = Blk::VedicExt;
    } else if block.as_str() == Blk::VerticalForms.property_value_name().abbr {
        blk = Blk::VerticalForms;
    } else if block.as_str() == Blk::Vithkuqi.property_value_name().abbr {
        blk = Blk::Vithkuqi;
    } else if block.as_str() == Blk::Vs.property_value_name().abbr {
        blk = Blk::Vs;
    } else if block.as_str() == Blk::VsSup.property_value_name().abbr {
        blk = Blk::VsSup;
    } else if block.as_str() == Blk::Wancho.property_value_name().abbr {
        blk = Blk::Wancho;
    } else if block.as_str() == Blk::WarangCiti.property_value_name().abbr {
        blk = Blk::WarangCiti;
    } else if block.as_str() == Blk::Yezidi.property_value_name().abbr {
        blk = Blk::Yezidi;
    } else if block.as_str() == Blk::YiRadicals.property_value_name().abbr {
        blk = Blk::YiRadicals;
    } else if block.as_str() == Blk::YiSyllables.property_value_name().abbr {
        blk = Blk::YiSyllables;
    } else if block.as_str() == Blk::Yijing.property_value_name().abbr {
        blk = Blk::Yijing;
    } else if block.as_str() == Blk::ZanabazarSquare.property_value_name().abbr {
        blk = Blk::ZanabazarSquare;
    } else if block.as_str() == Blk::ZnamennyMusic.property_value_name().abbr {
        blk = Blk::ZnamennyMusic;
    }

    for i in 0x0000u32..=0x10FFFFu32 {
        let cp = CodePoint::new(i).unwrap();
        if cp.blk() == blk {
            code_points.push(cp.to_string());
        }
    }

    json!({
        "unicode_version": UNICODE_VERSION.to_string(),
        "seshat_version": SESHAT_VERSION,
        "code_points": code_points,
        "range_begin": code_points.first(),
        "range_end": code_points.last(),
    })
}
