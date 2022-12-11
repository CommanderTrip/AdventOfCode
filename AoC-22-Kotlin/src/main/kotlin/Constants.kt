class Constants {

    companion object {
        /**
         * Get the path to the specified Challenge day and Test.
         *
         * @param adventDay Advent Code Day
         * @param isExample Use example input or Real input
         */
        fun getPath(adventDay: Int, isExample: Boolean): String {
            // Ensure day was a number between 1 - 25
            if (adventDay < 1 || adventDay > 25) throw Error("Challenge day must be between 1 and 25 inclusively.")

            // Get the advent day folder name
            val verifiedDayInput = if (adventDay in 1..9) {
                "day0".plus(adventDay.toString())
            } else {
                "day".plus(adventDay.toString())
            }

            // Get input file name
            val fileName = if (isExample) {
                "sample.txt"
            } else {
                "input.txt"
            }

            return System.getProperty("user.dir").plus("\\src\\main\\kotlin\\$verifiedDayInput\\$fileName")
        }
    }

}