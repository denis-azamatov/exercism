pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let children = vec!["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];

    let id = children.iter().position(|&x| x == student).unwrap();

    let mut flowers = vec![];

    for line in diagram.lines() {
        for i in 0..2 {
            let flower = match line.chars().nth(id * 2 + i).unwrap() {
                'G' => "grass",
                'C' => "clover",
                'R' => "radishes",
                'V' => "violets",
                _ => panic!("Unknown flower")
            };
            flowers.push(flower);
        }
    }

    flowers
}

#[cfg(test)]
mod test {
    use crate::kindrgarten::plants;

    #[test]
    fn garden_with_single_student() {
        let diagram = "RC\nGG";


        let student = "Alice";


        let expected = vec!["radishes", "clover", "grass", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn different_garden_with_single_student() {
        let diagram = "VC\nRC";


        let student = "Alice";


        let expected = vec!["violets", "clover", "radishes", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn garden_with_two_students() {
        let diagram = "VVCG\nVVRC";


        let student = "Bob";


        let expected = vec!["clover", "grass", "radishes", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn second_students_garden() {
        let diagram = "VVCCGG\nVVCCGG";


        let student = "Bob";


        let expected = vec!["clover", "clover", "clover", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn third_students_garden() {
        let diagram = "VVCCGG\nVVCCGG";


        let student = "Charlie";


        let expected = vec!["grass", "grass", "grass", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_alice_first_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Alice";


        let expected = vec!["violets", "radishes", "violets", "radishes"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_bob_second_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Bob";


        let expected = vec!["clover", "grass", "clover", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_charlie() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Charlie";


        let expected = vec!["violets", "violets", "clover", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_david() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "David";


        let expected = vec!["radishes", "violets", "clover", "radishes"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_eve() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Eve";


        let expected = vec!["clover", "grass", "radishes", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_fred() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Fred";


        let expected = vec!["grass", "clover", "violets", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_ginny() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Ginny";


        let expected = vec!["clover", "grass", "grass", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_harriet() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Harriet";


        let expected = vec!["violets", "radishes", "radishes", "violets"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_ileana() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Ileana";


        let expected = vec!["grass", "clover", "violets", "clover"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_joseph() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Joseph";


        let expected = vec!["violets", "clover", "violets", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_kincaid_second_to_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Kincaid";


        let expected = vec!["grass", "clover", "clover", "grass"];


        assert_eq!(plants(diagram, student), expected);
    }


    #[test]
    fn for_larry_last_students_garden() {
        let diagram = "VRCGVVRVCGGCCGVRGCVCGCGV\nVRCCCGCRRGVCGCRVVCVGCGCV";


        let student = "Larry";


        let expected = vec!["grass", "violets", "clover", "violets"];


        assert_eq!(plants(diagram, student), expected);
    }
}