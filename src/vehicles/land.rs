use juniper::FieldResult;

#[derive(Debug, GraphQLObject)]
#[graphql(description = "A car.")]
pub struct Car {
    pub name: String,
    pub model: String,
    pub year: i32,
    pub color: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A car.")]
struct NewCar {
    name: String,
    model: String,
    year: i32,
    color: String,
}
