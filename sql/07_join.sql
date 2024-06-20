SELECT * FROM users 
    FULL JOIN user_lecture 
        ON users.id=user_lecture.user_id
    FULL Join lecture
        ON user_lecture.lecture_id = lecture.id;