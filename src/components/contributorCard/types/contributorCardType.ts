export default interface contributorCardType{
    picture : string,
    pseudo  : string, 
    email   : string,
    networks?: {
        github? : string, 
        gitlab? : string,
        linkedin?: string 
    }
}