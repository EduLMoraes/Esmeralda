
ALTER TABLE Counts ADD CONSTRAINT FK_Counts_2
    FOREIGN KEY (id_user)
    REFERENCES Users (id_user)
    ON DELETE CASCADE;
 
ALTER TABLE History ADD CONSTRAINT FK_History_2
    FOREIGN KEY (id_old_count)
    REFERENCES Old_Counts (id_old_count)
    ON DELETE CASCADE;
 
ALTER TABLE History ADD CONSTRAINT FK_History_3
    FOREIGN KEY (id_count)
    REFERENCES Counts (id_count)
    ON DELETE CASCADE;
 
ALTER TABLE History ADD CONSTRAINT FK_History_4
    FOREIGN KEY (id_user)
    REFERENCES Users (id_user)
    ON DELETE CASCADE;
 
ALTER TABLE Investments ADD CONSTRAINT FK_Investments_2
    FOREIGN KEY (uid_people)
    REFERENCES People (uid_people)
    ON DELETE CASCADE;
 
ALTER TABLE FIIs ADD CONSTRAINT FK_FIIs_2
    FOREIGN KEY (last_yields_PK)
    REFERENCES last_yields (last_yields_PK)
    ON DELETE NO ACTION;
 
ALTER TABLE FIIs ADD CONSTRAINT FK_FIIs_3
    FOREIGN KEY (dates_yield_PK)
    REFERENCES dates_yield (dates_yield_PK)
    ON DELETE NO ACTION;
 
ALTER TABLE Address ADD CONSTRAINT FK_Address_2
    FOREIGN KEY (uid_property)
    REFERENCES Property (uid_property)
    ON DELETE CASCADE;
 
ALTER TABLE Bank ADD CONSTRAINT FK_Bank_2
    FOREIGN KEY (uid_people)
    REFERENCES People (uid_people)
    ON DELETE CASCADE;
 
ALTER TABLE People ADD CONSTRAINT FK_People_2
    FOREIGN KEY (id_addres)
    REFERENCES Address (id_addres)
    ON DELETE CASCADE;
 
ALTER TABLE People ADD CONSTRAINT FK_People_3
    FOREIGN KEY (id_user)
    REFERENCES Users (id_user)
    ON DELETE CASCADE;
 
ALTER TABLE People ADD CONSTRAINT FK_People_4
    FOREIGN KEY (provider)
    REFERENCES People (uid_people);
 
ALTER TABLE Receipts ADD CONSTRAINT FK_Receipts_2
    FOREIGN KEY (uid_property)
    REFERENCES Property (uid_property)
    ON DELETE CASCADE;
 
ALTER TABLE Goal ADD CONSTRAINT FK_Goal_2
    FOREIGN KEY (id_user)
    REFERENCES Users (id_user)
    ON DELETE CASCADE;
 
ALTER TABLE Investments_FIIs_Stock_Exchange_Shares ADD CONSTRAINT FK_Investments_FIIs_Stock_Exchange_Shares_1
    FOREIGN KEY (id_invest)
    REFERENCES Investments (id_invest)
    ON DELETE NO ACTION;
 
ALTER TABLE Investments_FIIs_Stock_Exchange_Shares ADD CONSTRAINT FK_Investments_FIIs_Stock_Exchange_Shares_2
    FOREIGN KEY (id_fii)
    REFERENCES FIIs (id_fii)
    ON DELETE CASCADE;
 
ALTER TABLE Investments_FIIs_Stock_Exchange_Shares ADD CONSTRAINT FK_Investments_FIIs_Stock_Exchange_Shares_3
    FOREIGN KEY (id_SES)
    REFERENCES Stock_Exchange_Shares (id_SES)
    ON DELETE CASCADE;