/// Tests for fractions
///   includes simple fractions and more complex fractions
///   also tests mixed fractions (implicit and explicit)
use crate::common::*;
use anyhow::Result;

#[test]
fn common_fraction_half() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "1 mezzo")?;
    return Ok(());

}

#[test]
fn common_fraction_thirds() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>2</mn> <mn>3</mn> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "2 terzi")?;
    return Ok(());

}

#[test]
fn common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>17</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "17 decimi")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "17 decimi")?;
    return Ok(());

}

#[test]
#[allow(non_snake_case)]
fn not_ClearSpeak_common_fraction_tenths() -> Result<()> {
    let expr = "<math>
                    <mfrac> <mn>89</mn> <mn>10</mn> </mfrac>
                </math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "89 fratto 10")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "89 decimi")?;
    return Ok(());

}

#[test]
fn non_simple_fraction() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mfrac>
        <mrow>
        <mi>x</mi><mo>+</mo><mi>y</mi></mrow>
        <mrow>
        <mi>x</mi><mo>-</mo><mi>y</mi></mrow>
        </mfrac>
        </mrow>
    </math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "frazione  con numeratore; x più y; e denominatore x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Ordinal")], expr, "frazione con numeratore; x più y; e denominatore x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Over")], expr, "x più y fratto x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "FracOver")], expr, "frazione x più y fratto x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "General")], expr, "frazione con numeratore; x più y; e denominatore x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "EndFrac")], expr, "frazione con numeratore; x più y; e denominatore x meno y; fine frazione")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "GeneralEndFrac")], expr, "frazione con numeratore; x più y; e denominatore x meno y; fine frazione")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "OverEndFrac")], expr, "x più y fratto x meno y, fine frazione")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Per")], expr, "x più y fratto x meno y")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Verbose"),("ClearSpeak_Fractions", "Auto")], expr, "frazione con numeratore; x più y; e denominatore x meno y; fine frazione")?;
    return Ok(());

}

#[test]
fn frac_with_units() -> Result<()> {
    let expr = "
    <math>
        <mrow>
        <mn>62</mn>
        <mfrac>
        <mi intent=':unit'>km</mi>
        <mi intent=':unit'>hr</mi>
        </mfrac>
        </mrow>
    </math>";
    test("it", "ClearSpeak", expr, "62 chilometri all'ora")?;
    return Ok(());

}


#[test]
fn mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>1</mn> <mn>2</mn> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "3 più 1 mezzo")?;
    return Ok(());

}

#[test]
fn explicit_mixed_number() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mo>&#x2064;</mo>
                    <mfrac> <mn>1</mn> <mn>8</mn> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "3 più 1 ottavo")?;
    return Ok(());

}

#[test]
fn mixed_number_big() -> Result<()> {
    let expr = "<math>
                    <mn>3</mn>
                    <mfrac> <mn>7</mn> <mn>83</mn> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "3 più 7 fratto 83")?;
    return Ok(());

}

#[test]
fn simple_text() -> Result<()> {
    let expr = "<math>
    <mfrac> <mi>rise</mi> <mi>run</mi> </mfrac>
                </math>";
    test("it", "ClearSpeak", expr, "dislivello fratto distanza")?;
    return Ok(());

}

#[test]
fn number_and_text() -> Result<()> {
    let expr = "<math>
            <mfrac>
            <mrow>
                <mn>2</mn><mtext>chilometri</mtext></mrow>
            <mrow>
                <mn>3</mn><mtext>litri</mtext></mrow>
            </mfrac>
        </math>";
    test("it", "ClearSpeak", expr, "2 chilometri fratto 3 litri")?;
    return Ok(());

}


#[test]
fn nested_simple_fractions() -> Result<()> {
    let expr = "<math>
                <mrow>
                <mfrac>
                    <mrow>
                    <mfrac>
                        <mn>1</mn>
                        <mn>2</mn>
                    </mfrac>
                    </mrow>
                    <mrow>
                    <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                    </mfrac>
                    </mrow>
                </mfrac>
                </mrow>
            </math>";
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "Auto")], expr, "1 mezzo fratto 2 terzi")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "Ordinal")], expr, "1 mezzo fratto 2 terzi")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "Over")], expr, "1 fratto 2 fratto 2 fratto 3")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "FracOver")], expr,
            "frazione; frazione; 1 fratto 2; fratto; frazione; 2 fratto 3")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "General")], expr,
            "frazione con numeratore; frazione con numeratore 1; e denominatore 2; e denominatore; frazione con numeratore 2; e denominatore 3")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "EndFrac")], expr, "1 mezzo fratto 2 terzi")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "GeneralEndFrac")], expr,
            "frazione con numeratore; frazione con numeratore 1; e denominatore 2; fine frazione; e denominatore frazione con numeratore 2; e denominatore 3; fine frazione; fine frazione")?;
    test_prefs("it", "ClearSpeak", vec![("ClearSpeak_Fractions", "OverEndFrac")], expr,
            "1 fratto 2, fine frazione, fratto 2 fratto 3, fine frazione; fine frazione")?;
            return Ok(());

}


#[test]
fn semi_nested_fraction() -> Result<()> {
    let expr = "<math>
                <mrow>
                        <mfrac>
                        <mrow>
                        <mfrac>
                        <mn>2</mn>
                        <mn>3</mn>
                        </mfrac>
                        <mi>x</mi>
                    </mrow>
                    <mn>6</mn>
                    </mfrac>
                </mrow>
                </math>";
    test("it", "ClearSpeak", expr, "2 terzi x fratto 6")?;
    return Ok(());

}

#[test]
fn general_nested_fraction() -> Result<()> {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mn>10</mn>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("it", "ClearSpeak", expr, "frazione con numeratore; 10 fratto n; e denominatore 2 fratto n")?;
    return Ok(());

}

#[test]
fn complex_nested_fraction() -> Result<()> {
    let expr = "
    <math>
    <mrow>
        <mfrac>
        <mrow>
        <mfrac>
            <mrow> <mi>n</mi> <mo>+</mo> <mn>10</mn> </mrow>
            <mi>n</mi>
        </mfrac>
        </mrow>
        <mrow>
        <mfrac>
        <mn>2</mn>
        <mi>n</mi>
        </mfrac>
        </mrow>
        </mfrac>
        </mrow>
    </math>
                    ";
    test("it", "ClearSpeak", expr, " frazione con numeratore; frazione con numeratore; n più 10; e denominatore n; e denominatore 2 fratto n")?;
    return Ok(());

}

#[test]
fn simple_function() -> Result<()> {
    let expr = "<math><mfrac><mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow><mn>2</mn></mfrac></math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f di x fratto 2")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f di x fratto 2, fine frazione")?;
    return Ok(());

}

#[test]
fn function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr, "f di x fratto g di x")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr, "f di x fratto g di x, fine frazione")?;
    return Ok(());

}

#[test]
fn non_simple_function_over_function() -> Result<()> {
    let expr = "<math><mfrac>
            <mrow><mi>f</mi><mo>(</mo><mi>x</mi><mo>+</mo><mn>1</mn><mo>)</mo></mrow>
            <mrow><mi>g</mi><mo>(</mo><mi>x</mi><mo>)</mo></mrow>
        </mfrac></math>";
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Medium"), ("ClearSpeak_Fractions", "Auto")], expr,
             "frazione con numeratore; f di, aperta parentesi tonda x più 1, chiusa parentesi tonda; e denominatore g di x")?;
    test_prefs("it", "ClearSpeak", vec![("Verbosity", "Verbose"), ("ClearSpeak_Fractions", "Auto")], expr,
             "frazione con numeratore; f di, aperta parentesi tonda x più 1, chiusa parentesi tonda; e denominatore g di x; fine frazione")?;
             return Ok(());

}

#[test]
fn binomial() -> Result<()> {
    let expr = "<math>
                    <mn>2</mn>
                    <mo>(</mo>
                    <mfrac linethickness='0'> <mn>7</mn> <mn>3</mn> </mfrac>
                    <mo>)</mo>
                </math>";
    test("it", "ClearSpeak", expr, "2 per 7 fratto 3")?;
    return Ok(());

}
