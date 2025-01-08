import type { JSX } from 'react'
import ContributorCard from '../components/contributorCard/contributorCard'

export default function IndexPage(): JSX.Element {
	return (
		<>
		<h1>Homepage</h1>
		<ContributorCard 
			picture={'spacecodeur.png'} 
			pseudo={'spacecodeur'} 
			email={'spacecodeur@gmail.com'} 
			networks={
				{
					linkedin: "https://www.linkedin.com/in/spacecodeur",
					gitlab: "https://gitlab.com/spacecodeur",
					github: "https://github.com/spacecodeur"
				}
			} />
		<ContributorCard picture={''} pseudo={'hazefury'} email={''} />
		</>
	)
}
