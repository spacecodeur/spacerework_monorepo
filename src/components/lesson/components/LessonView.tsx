type PropsType = {
    content: String|null
}

export default function LessonView({content}: PropsType){
    return <section dangerouslySetInnerHTML={{ __html: content }}>
    </section>
}