SELECT * FROM users 
    JOIN user_lecture 
        ON users.id=user_lecture.user_id;