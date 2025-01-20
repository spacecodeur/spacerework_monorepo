import type { JSX } from 'react'
import ContributorCard from '../components/contributorCard/contributorCard'
import type { TuonoProps } from 'tuono'


export default function IndexPage({
    data,
    isLoading,
}: TuonoProps<string>): JSX.Element {
	if (isLoading) {
        return <h1>Loading...</h1>
    }


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
		<div>
			<p>{data}</p>
		</div>
		</>
	)
}
