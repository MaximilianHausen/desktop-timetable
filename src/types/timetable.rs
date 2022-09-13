#[derive(PartialEq, Clone)]
pub struct Timetable {
    pub times: Vec<Vec<String>>,
    pub columns: Vec<TimetableColumn>,
}

#[derive(PartialEq, Clone)]
pub struct TimetableColumn {
    pub name: String,
    pub lessons: Vec<Lesson>,
}

#[derive(PartialEq, Clone)]
pub struct Lesson {
    pub subject: Subject,
    pub status: LessonStatus,
}

#[derive(PartialEq, Clone)]
pub struct Subject {
    pub full_name: String,
    pub short_name: String,
    pub color: (u8, u8, u8),
}

#[derive(PartialEq, Clone)]
pub enum LessonStatus {
    Normal,
    Cancelled,
    Replaced(Box<Lesson>),
}
