# Smart Book Recommendations with Graph-Based Relationships

This project is designed to provide personalized book recommendations by analyzing reading patterns of different users. Instead of relying on traditional rating systems, the algorithm looks at shared reading interests between users to suggest books that might be a great match.

## How Does It Work?

When a user logs in, the system:

1. Finds books the user has already read

    ```MATCH (u:User {id: $id})-[:HAS_READ]->(b:Book)```

    This identifies all books associated with the given user.

2. Finds other users who have read the same books

    ```MATCH (similarUser:User)-[:HAS_READ]->(b) WHERE similarUser <> u```

    The algorithm looks for similar users who have read at least one of the same books as the current user.

3. Identifies books read by similar users

    ```MATCH (similarUser)-[:HAS_READ]->(rec:Book)```

    It finds books that these similar users have read.

4. Filters out books the user has already read

    ```WHERE NOT (u)-[:HAS_READ]->(rec)```

    Ensures that recommendations only include new books the user hasn’t read yet.

5. Ranks the recommendations

    ```RETURN rec.id, rec.title, rec.author, rec.genre, rec.cover, COUNT(similarUser) AS score ORDER BY score DESC LIMIT 5```

    The graph database counts how many similar users have read each book and prioritizes the most popular ones among them, returning the top 5 recommendations.

## Technology Stack

- Rust + Rocket: The backend is built with Rust using the Rocket framework for a fast and efficient REST API.

- Neo4j (Graph Database): A relationship-based database is used to store users, books, and reading connections, making recommendations highly efficient.

- Angular Frontend: The UI interacts with the Rust API, displaying book recommendations in a user-friendly way.

## Why This Approach?

Instead of relying on explicit ratings, this method takes advantage of social reading behavior. If multiple people with similar tastes enjoy a book, it’s likely that the user will too! By using a graph database, we can efficiently find these patterns and deliver highly relevant book recommendations.